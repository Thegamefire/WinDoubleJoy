use btleplug::api::{bleuuid::BleUuid, Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;

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
                    println!("{:?}", manufacturer_data);
                }
            }
            _ => {}
        }
    }
}
