use tokio::{join, sync::mpsc::Receiver, task::JoinHandle};
use tracing::info;
use vigem_client::{Client, TargetId, XGamepad, Xbox360Wired};

use crate::{bluetooth::state::ControllerState, vigem::apply::Apply};

pub struct VigemManager {
    controller: Xbox360Wired<Client>,
}

impl VigemManager {
    pub fn new() -> Self {
        info!("connecting vigem client");
        let client = Client::connect().unwrap();
        let mut controller = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);

        info!("plugging in vigem controller and waiting until ready");
        controller.plugin().unwrap();
        controller.wait_ready().unwrap();

        Self { controller }
    }

    pub fn start_thread(
        mut self,
        mut one: Receiver<ControllerState>,
        mut two: Receiver<ControllerState>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            info!("starting vigem thread");
            while let (Some(controller1), Some(controller2)) = join!(one.recv(), two.recv()) {
                let mut pad = XGamepad::default();
                pad.apply(controller1);
                pad.apply(controller2);

                self.controller.update(&pad).unwrap();
            }
            info!("vigem thread ended");
        })
    }
}
