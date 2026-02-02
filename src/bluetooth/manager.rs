use std::collections::HashMap;

use btleplug::{
    Result as BtleResult,
    api::{Central, CentralEvent::*, Manager as _, Peripheral, ScanFilter, WriteType},
    platform::{Adapter, Manager, PeripheralId},
};
use futures::StreamExt;
use tracing::{debug, info, instrument};
use uuid::{Uuid, uuid};

const JOYCONLEFT_UUID: Uuid = uuid!("cc1bbbb5-7354-4d32-a716-a81cb241a32a");
const JOYCONRIGHT_UUID: Uuid = uuid!("d5a9e01e-2ffc-4cca-b20c-8b67142bf442");
const COMMAND_CHARACTERISTIC_UUID: Uuid = uuid!("649d4ac9-8eb7-4e6c-af44-1ea54fe5f005");

const NINTENDO_MANUFACTURER: [u8; 24] = [
    1, 0, 3, 126, 5, 103, 32, 0, 1, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0,
];

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
    pub async fn start_scan(&self) {
        info!("scanning: starting");
        self.adapter.start_scan(ScanFilter::default());
        info!("scanning: started");
    }

    /// listen to events on the adapter and handle them
    pub async fn run_eventloop(&self) {
        info!("eventloop: start listening to adapter events");
        while let Some(event) = self.adapter.events().await.unwrap().next().await {
            debug!("eventloop: found event: {:?}", event);
            match event {
                ManufacturerDataAdvertisement {
                    id,
                    manufacturer_data,
                } => {
                    debug!(
                        "eventloop: data for key 0x0553: {:?}",
                        manufacturer_data.get(&0x0553)
                    );
                    if let Some(data) = manufacturer_data.get(&0x0553)
                        && data == &NINTENDO_MANUFACTURER
                    {
                        self.connect_to_peripheral(id).await.unwrap();
                    }
                }
                DeviceConnected(id) => self.handle_connect(id).await.unwrap(),
                _ => {}
            }
        }
        info!("eventloop: finished");
    }

    /// connect to a peripheral using it's id
    async fn connect_to_peripheral(&self, id: PeripheralId) -> BtleResult<()> {
        info!("connecting to peripheral with id {id}");
        self.adapter.peripheral(&id).await?.connect().await?;
        self.adapter
            .peripheral(&id)
            .await?
            .discover_services()
            .await?;
        info!("connected to peripheral with id {id}");
        Ok(())
    }

    /// handling for the connect event
    async fn handle_connect(&self, id: PeripheralId) -> BtleResult<()> {
        info!("handling a connection event for peripheral {id}");
        let peripheral = self.adapter.peripheral(&id).await?;
        for characteristic in peripheral.characteristics() {
            match characteristic.service_uuid {
                JOYCONLEFT_UUID => {
                    info!("left joycon found");
                    peripheral.subscribe(&characteristic).await.unwrap();
                    peripheral
                        .write(
                            &peripheral
                                .characteristics()
                                .iter()
                                .find(|ch| ch.uuid == COMMAND_CHARACTERISTIC_UUID)
                                .unwrap(),
                            &[
                                0x09, 0x91, 0x00, 0x07, 0x00, 0x08, 0x00, 0x00, 0b1001, 0x00, 0x00,
                                0x00, 0x00, 0x00, 0x00, 0x00,
                            ],
                            WriteType::WithoutResponse,
                        )
                        .await
                        .unwrap();
                }
                JOYCONRIGHT_UUID => {
                    info!("right joycon found");
                    peripheral.subscribe(&characteristic).await?;
                    let mut stream = peripheral.notifications().await?;
                    // spawn a thread to listen to the message stream
                    tokio::spawn(async move {
                        info!("right joycon tread started");
                        while let Some(msg) = stream.next().await {
                            dbg!(msg);
                        }
                        info!("right joycon thread ended");
                    });
                }
                _ => debug!("skipped characteristic {}", characteristic),
            }
        }

        info!("finished handling connect");
        Ok(())
    }
}
