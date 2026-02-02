use btleplug::api::{bleuuid::BleUuid, Central, CentralEvent, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use uuid::{uuid, Uuid};

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
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
                if (manufacturer_data.contains_key(&0x0553)
                    && manufacturer_data[&0x0553]==[1, 0, 3, 126, 5, 103, 32, 0, 1, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0]) {
                    central.peripheral(&id).await.unwrap().connect().await.unwrap();
                }
            }
            CentralEvent::DeviceConnected(id) => {
                for characteristic in  central.peripheral(&id).await.unwrap().characteristics() {
                    if (characteristic.service_uuid==uuid!("cc1bbbb5-7354-4d32-a716-a81cb241a32a")) {
                        // JoyConLeft
                        central.peripheral(&id).await.unwrap().subscribe(&characteristic).await.unwrap();
                    } else if (characteristic.service_uuid==uuid!("d5a9e01e-2ffc-4cca-b20c-8b67142bf442")) {
                        // JoyConRight
                        central.peripheral(&id).await.unwrap().subscribe(&characteristic).await.unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}
