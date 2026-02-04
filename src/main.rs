use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::bluetooth::manager::BluetoothManager;
#[cfg(feature = "vigem")]
use crate::vigem::client::VigemManager;

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

    info!("connecting to controller 2");
    let controller2 = manager.connect_controller().await.unwrap();

    #[cfg(feature = "vigem")]
    if let (Some(c1), Some(c2)) = (controller1, controller2) {
        info!("both controllers connected, starting vigem");
        let manager = VigemManager::new();
        let handle = manager.start_thread(c1.update_receiver, c2.update_receiver);
        handle.await.unwrap();
    }
}
