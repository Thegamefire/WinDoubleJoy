use std::collections::HashMap;

use btleplug::{
    Result as BtleResult,
    api::{Central, CentralEvent::*, Manager as _, Peripheral, ScanFilter, WriteType},
    platform::{Adapter, Manager, PeripheralId},
};
use futures::StreamExt;
use uuid::{Uuid, uuid};

const JOYCONLEFT_UUID: Uuid = uuid!("cc1bbbb5-7354-4d32-a716-a81cb241a32a");
const JOYCONRIGHT_UUID: Uuid = uuid!("d5a9e01e-2ffc-4cca-b20c-8b67142bf442");
const COMMAND_CHARACTERISTIC_UUID: Uuid = uuid!("649d4ac9-8eb7-4e6c-af44-1ea54fe5f005");

const NINTENDO_MANUFACTURER: [u8; 24] = [
    1, 0, 3, 126, 5, 103, 32, 0, 1, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0,
];

pub struct BluetoothManager {
    manager: Manager,
    adapter: Adapter,
}

impl BluetoothManager {
    /// setup the manager and select the first bluetooth adapter
    pub async fn new() -> Self {
        let manager = Manager::new().await.unwrap();
        let adapters = manager.adapters().await.unwrap();
        let adapter = adapters.into_iter().nth(0).unwrap();

        Self { manager, adapter }
    }

    /// start scanning for bluetooth devices
    pub async fn start_scan(&self) {
        self.adapter.start_scan(ScanFilter::default());
    }

    /// listen to events on the adapter and handle them
    pub async fn run_eventloop(&self) {
        while let Some(event) = self.adapter.events().await.unwrap().next().await {
            match event {
                ManufacturerDataAdvertisement {
                    id,
                    manufacturer_data,
                } => {
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
    }

    /// connect to a peripheral using it's id
    async fn connect_to_peripheral(&self, id: PeripheralId) -> BtleResult<()> {
        self.adapter.peripheral(&id).await?.connect().await?;
        self.adapter
            .peripheral(&id)
            .await?
            .discover_services()
            .await?;
        Ok(())
    }

    /// handling for the connect event
    async fn handle_connect(&self, id: PeripheralId) -> BtleResult<()> {
        let peripheral = self.adapter.peripheral(&id).await?;
        for characteristic in peripheral.characteristics() {
            match characteristic.service_uuid {
                JOYCONLEFT_UUID => {
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
                    peripheral.subscribe(&characteristic).await.unwrap();
                    let mut stream = peripheral.notifications().await.unwrap();
                    while let Some(msg) = stream.next().await {
                        dbg!(msg);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
