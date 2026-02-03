pub enum Commands {
    SetLED(u8),
    SendVibration
}

impl Commands {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Commands::SetLED(led) => {
                Vec::from([
                    0x09, 0x91, 0x00, 0x07, 0x00, 0x08, 0x00, 0x00, led, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ])
            }
            Commands::SendVibration => {
                Vec::from([
                    0x0A, 0x91, 0x01, 0x02,
                    0x00, 0x04, 0x00, 0x00,
                    0x03, 0x00, 0x00, 0x00
                ])
            }
        }
    }
}