use vigem_client::{Client, TargetId, Xbox360Wired};

use crate::vigem;

pub struct Vigem {
    controller: Xbox360Wired<Client>,
    state: XGamepad,
}

impl Vigem {
    fn new() -> Self {
        let client = Client::connect().unwrap();
        let mut controller = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);

        controller.plugin().unwrap();

        controller.wait_ready().unwrap();

        Self { controller }
    }

    fn read_loop(self, one: Receiver, two: Receiver) {
        tokio::spawn(async move {
            let pad = XGamepad::new();
            while let (Some(msg_one), Some(msg_two)) = join!(one.recv(), two.recv()) {
                if let Some(msg) = msg_one {
                    msg.apply_to(&mut pad);
                }
                if let Some(msg) = msg_two {
                    msg.apply_to(&mut pad);
                }

                self.controller.update(&pad);
            }
        });
    }
}
