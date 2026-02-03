use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::bluetooth::manager::BluetoothManager;

pub mod bluetooth;
#[cfg(feature = "vigem")]
pub mod vigem;

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    info!("starting");
    let manager = BluetoothManager::new().await;

    info!("connecting to controller 1");
    let controller1 = manager.connect_controller().await.unwrap();
    dbg!(&controller1);
    if let Some(mut connection) = controller1 {
        let msg = connection.update_receiver.recv().await;
        dbg!(msg);
    }

    info!("connecting to controller 2");
    let controller2 = manager.connect_controller().await.unwrap();
    dbg!(&controller2);
    if let Some(mut connection) = controller2 {
        let msg = connection.update_receiver.recv().await;
        dbg!(msg);
    }
}
