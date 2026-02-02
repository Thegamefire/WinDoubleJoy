use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::bluetooth::manager::BluetoothManager;

pub mod bluetooth;

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    info!("starting");
    let manager = BluetoothManager::new().await;
    manager.start_scan().await;
    manager.run_eventloop().await;
}
