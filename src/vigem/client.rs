use vigem_client::{Client, TargetId, Xbox360Wired};

use crate::vigem;

pub struct Vigem {
    controller: Xbox360Wired<Client>,
}

impl Vigem {
    fn new() -> Self {
        let client = Client::connect().unwrap();
        let mut controller = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);

        controller.plugin().unwrap();

        controller.wait_ready().unwrap();

        Self { controller }
    }
}
