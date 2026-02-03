use std::collections::HashMap;

use crate::bluetooth::{
    commands::Commands,
    controller::ControllerConnection,
    state::{ControllerState, LeftJoyConState, RightJoyConState},
};
use btleplug::{
    Result as BtleResult,
    api::{
        Central,
        CentralEvent::{self, *},
        Characteristic, Manager as _, Peripheral as _, ScanFilter, ValueNotification, WriteType,
    },
    platform::{Adapter, Manager, Peripheral, PeripheralId},
};
use futures::StreamExt;
use tokio::sync::mpsc::{self, error::TrySendError};
use tracing::{debug, info, instrument, trace};
use uuid::{Uuid, uuid};

const JOYCONLEFT_UUID: Uuid = uuid!("cc1bbbb5-7354-4d32-a716-a81cb241a32a");
const JOYCONRIGHT_UUID: Uuid = uuid!("d5a9e01e-2ffc-4cca-b20c-8b67142bf442");
const COMMAND_CHARACTERISTIC_UUID: Uuid = uuid!("649d4ac9-8eb7-4e6c-af44-1ea54fe5f005");

const NINTENDO_MANUFACTURER_PREFIX: [u8; 5] = [1, 0, 3, 126, 5];

#[derive(Debug)]
pub struct BluetoothManager {
    manager: Manager,
    adapter: Adapter,
}

impl BluetoothManager {
    /// setup the manager and select the first bluetooth adapter
    pub async fn new() -> Self {
        info!("manager and adapter: getting");
        let manager = Manager::new().await.unwrap();
        let adapters = manager.adapters().await.unwrap();
        let adapter = adapters.into_iter().nth(0).unwrap();
        info!("manager and adapter: found");

        Self { manager, adapter }
    }

    /// start scanning for bluetooth devices
    async fn start_scan(&self) {
        self.adapter
            .start_scan(ScanFilter::default())
            .await
            .unwrap();
        info!("scanning: started");
    }

    /// stop scanning for bluetooth devices
    async fn stop_scan(&self) {
        self.adapter.stop_scan().await.unwrap();
        info!("scanning: started");
    }

    /// listen to events on the adapter and handle them
    pub async fn connect_controller(&self) -> BtleResult<Option<ControllerConnection>> {
        self.start_scan().await;
        info!("connecting controller: start listening to adapter events");
        while let Some(event) = self.adapter.events().await.unwrap().next().await {
            trace!("eventloop: event {:?}", event);
            match event {
                ManufacturerDataAdvertisement {
                    id,
                    manufacturer_data,
                } => {
                    if let Some(data) = manufacturer_data.get(&0x0553)
                        && data.starts_with(&NINTENDO_MANUFACTURER_PREFIX)
                    {
                        let peripheral = self.adapter.peripheral(&id).await?;
                        self.connect_peripheral(&peripheral).await.unwrap();
                        if let Some(controller) = self.handle_connect(&peripheral).await.unwrap() {
                            return Ok(Some(controller));
                        }
                        self.disconnect_peripheral(&peripheral).await?;
                    }
                }
                DeviceConnected(id) => info!("device connected"),
                _ => {}
            }
        }
        self.stop_scan().await;
        info!("connecting controller: finished");
        Ok(None)
    }

    /// connect to a peripheral using it's id
    async fn connect_peripheral(&self, peripheral: &Peripheral) -> BtleResult<()> {
        info!("connecting to peripheral {:?}", peripheral);
        peripheral.connect().await?;
        peripheral.discover_services().await?;
        info!("connected to peripheral {:?}", peripheral);
        Ok(())
    }

    /// connect to a peripheral using it's id
    async fn disconnect_peripheral(&self, peripheral: &Peripheral) -> BtleResult<()> {
        info!("disconnecting from peripheral {:?}", peripheral);
        peripheral.disconnect().await?;
        Ok(())
    }

    /// handling for the connect event
    async fn handle_connect(
        &self,
        peripheral: &Peripheral,
    ) -> BtleResult<Option<ControllerConnection>> {
        info!(
            "handling a connection event for peripheral {:?}",
            peripheral
        );
        for characteristic in peripheral.characteristics() {
            match characteristic.uuid {
                JOYCONLEFT_UUID => {
                    info!("left joycon found");
                    self.initialize_joycon(&peripheral).await.unwrap();
                    return Ok(Some(
                        subscribe_and_listen(peripheral, characteristic, |msg| {
                            ControllerState::Left(LeftJoyConState::from(msg))
                        })
                        .await?,
                    ));
                }
                JOYCONRIGHT_UUID => {
                    info!("right joycon found");
                    self.initialize_joycon(&peripheral).await.unwrap();
                    return Ok(Some(
                        subscribe_and_listen(peripheral, characteristic, |msg| {
                            ControllerState::Right(RightJoyConState::from(msg))
                        })
                        .await?,
                    ));
                }
                _ => debug!("skipped characteristic {}", characteristic),
            }
        }

        info!("finished handling connect");
        Ok(None)
    }

    async fn initialize_joycon(&self, peripheral: &Peripheral) -> BtleResult<()> {
        if let Some(command_characteristic) = peripheral
            .characteristics()
            .iter()
            .find(|ch| ch.uuid == COMMAND_CHARACTERISTIC_UUID)
        {
            info!("writing to command");
            peripheral
                .write(
                    command_characteristic,
                    &Commands::SetLED(0b1001).to_bytes(),
                    WriteType::WithoutResponse,
                )
                .await?;

            peripheral
                .write(
                    command_characteristic,
                    &Commands::SendVibration.to_bytes(),
                    WriteType::WithoutResponse,
                )
                .await?;
        }

        Ok(())
    }
}

async fn subscribe_and_listen<F>(
    peripheral: &Peripheral,
    characteristic: Characteristic,
    mapper: F,
) -> BtleResult<ControllerConnection>
where
    F: Fn(ValueNotification) -> ControllerState + Send + Sync + 'static,
{
    info!("subscribing to {}", characteristic);
    peripheral.subscribe(&characteristic).await?;
    let mut stream = peripheral.notifications().await?;
    // spawn a thread to listen to the message stream
    let (sender, receiver) = mpsc::channel::<ControllerState>(2);
    let handle = tokio::spawn(async move {
        info!("joycon tread started");
        while let Some(msg) = stream.next().await {
            if let Err(TrySendError::Closed(_)) = sender.try_send(mapper(msg)) {
                break;
            }
        }
        info!("joycon thread ended");
    });

    return Ok(ControllerConnection {
        read_thread: handle,
        update_receiver: receiver,
    });
}
