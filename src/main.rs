use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use uuid::{Uuid, uuid};

pub mod bluetooth;

const JOYCONLEFT_UUID: Uuid = uuid!("cc1bbbb5-7354-4d32-a716-a81cb241a32a");
const JOYCONRIGHT_UUID: Uuid = uuid!("d5a9e01e-2ffc-4cca-b20c-8b67142bf442");

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

fn get_command_characteristic(peripheral: &Peripheral) -> Option<Characteristic> {
    for characteristic in peripheral.characteristics() {
        if characteristic.uuid == uuid!("649d4ac9-8eb7-4e6c-af44-1ea54fe5f005") {
            return Some(characteristic);
        }
    }
    return None;
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let manager = Manager::new().await.unwrap();
    let central = get_central(&manager).await;
    let mut events = central.events().await.unwrap();

    central.start_scan(ScanFilter::default()).await.unwrap();

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::ManufacturerDataAdvertisement {
                id,
                manufacturer_data,
            } => {
                if let Some(data) = manufacturer_data.get(&0x0553)
                    && data
                        == &[
                            1, 0, 3, 126, 5, 103, 32, 0, 1, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0,
                            0, 0,
                        ]
                {
                    central
                        .peripheral(&id)
                        .await
                        .unwrap()
                        .connect()
                        .await
                        .unwrap();
                    central
                        .peripheral(&id)
                        .await
                        .unwrap()
                        .discover_services()
                        .await
                        .unwrap();
                }
            }
            CentralEvent::DeviceConnected(id) => {
                let peripheral = central.peripheral(&id).await.unwrap();
                for characteristic in peripheral.characteristics() {
                    match characteristic.service_uuid {
                        JOYCONLEFT_UUID => {
                            // JoyConLeft
                            peripheral.subscribe(&characteristic).await.unwrap();
                            peripheral
                                .write(
                                    &get_command_characteristic(&peripheral).unwrap(),
                                    &[
                                        0x09, 0x91, 0x00, 0x07, 0x00, 0x08, 0x00, 0x00, 0b1001,
                                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                    ],
                                    WriteType::WithoutResponse,
                                )
                                .await
                                .unwrap();
                        }
                        JOYCONRIGHT_UUID => {
                            // JoyConRight
                            peripheral.subscribe(&characteristic).await.unwrap();
                            let mut stream = peripheral.notifications().await.unwrap();
                            while let Some(msg) = stream.next().await {
                                dbg!(msg);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
