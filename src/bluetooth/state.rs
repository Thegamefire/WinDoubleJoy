use btleplug::api::ValueNotification;

#[derive(Debug)]
pub struct ControllerState {
    pub stick_right: bool,
    pub plus: bool,
    pub zr: bool,
    pub r: bool,
    pub x: bool,
    pub y: bool,
    pub a: bool,
    pub b: bool,
    pub sl_right: bool,
    pub sr_right: bool,
    pub c: bool,
    pub home: bool,
    pub stick_rx: f32,
    pub stick_ry: f32,
    pub stick_left: bool,
    pub minus: bool,
    pub zl: bool,
    pub l: bool,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub sl_left: bool,
    pub sr_left: bool,
    pub capture: bool,
    pub stick_lx: f32,
    pub stick_ly: f32,
    pub gl: bool,
    pub gr: bool,

    // Motion Data
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
}

impl From<ValueNotification> for ControllerState {
    fn from(notification: ValueNotification) -> Self {
        let buttons = &notification.value[0x04..0x08];
        let motion_data = &notification.value[0x2A..0x3C];
        let (lx, ly) = decode_stick_data(&notification.value[0xA..0xD]);
        let (rx, ry) = decode_stick_data(&notification.value[0xD..0x10]);
        ControllerState {
            y: (buttons[0] & 0x01) > 0,
            x: (buttons[0] & 0x02) > 0,
            b: (buttons[0] & 0x04) > 0,
            a: (buttons[0] & 0x08) > 0,
            sr_right: (buttons[0] & 0x10) > 0,
            sl_right: (buttons[0] & 0x20) > 0,
            r: (buttons[0] & 0x40) > 0,
            zr: (buttons[0] & 0x80) > 0,

            minus: (buttons[1] & 0x01) > 0,
            plus: (buttons[1] & 0x02) > 0,
            stick_right: (buttons[1] & 0x04) > 0,
            stick_left: (buttons[1] & 0x08) > 0,
            home: (buttons[1] & 0x10) > 0,
            capture: (buttons[1] & 0x20) > 0,
            c: (buttons[1] & 0x40) > 0,

            down: (buttons[2] & 0x01) > 0,
            up: (buttons[2] & 0x02) > 0,
            right: (buttons[2] & 0x04) > 0,
            left: (buttons[2] & 0x08) > 0,
            sr_left: (buttons[2] & 0x10) > 0,
            sl_left: (buttons[2] & 0x20) > 0,
            l: (buttons[2] & 0x40) > 0,
            zl: (buttons[2] & 0x80) > 0,

            gr: (buttons[3] & 0x01) > 0,
            gl: (buttons[3] & 0x02) > 0,

            stick_lx: lx,
            stick_ly: ly,
            stick_rx: rx,
            stick_ry: ry,

            accel_x: 0.0,
            accel_y: 0.0,
            accel_z: 0.0,
            gyro_x: 0.0,
            gyro_y: 0.0,
            gyro_z: 0.0,
        }
    }
}

fn decode_stick_data(data: &[u8]) -> (f32, f32) {
    const X_STICK_MIN: f32 = 780.0;
    const X_STICK_MAX: f32 = 3260.0;
    const Y_STICK_MIN: f32 = 820.0;
    const Y_STICK_MAX: f32 = 3250.0;

    let x_raw = (((data[1] & 0x0F) as u16) << 8) | data[0] as u16;
    let y_raw = ((data[2] as u16) << 4) | ((data[1] & 0xF0) as u16) >> 4;

    let mut x = ((x_raw as f32) - X_STICK_MIN) / (X_STICK_MAX - X_STICK_MIN);
    let mut y = ((y_raw as f32) - Y_STICK_MIN) / (Y_STICK_MAX - Y_STICK_MIN);

    x = x.clamp(0.0, 1.0) * 2.0 - 1.0;
    y = y.clamp(0.0, 1.0) * 2.0 - 1.0;

    return (x, y);
}
