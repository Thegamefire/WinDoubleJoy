use tokio::{sync::mpsc::Receiver, task::JoinHandle};

use crate::bluetooth::state::ControllerState;

pub enum Controller {
    JoyconRight,
    JoyconLeft,
}

#[derive(Debug)]
pub struct ControllerConnection {
    pub read_thread: JoinHandle<()>,
    pub update_receiver: Receiver<ControllerState>,
}
