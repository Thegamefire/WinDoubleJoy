use crate::bluetooth::manager::BluetoothManager;

pub mod bluetooth;

#[tokio::main]
async fn main() {
    println!("starting");
    let manager = BluetoothManager::new().await;
    manager.start_scan().await;
    manager.run_eventloop().await;
}
