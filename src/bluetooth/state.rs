use btleplug::api::ValueNotification;
use vigem_client::{XButtons, XGamepad};


#[derive(Debug)]
pub struct ControllerState {
    stick_right: bool,
    plus: bool,
    zr: bool,
    r: bool,
    x: bool,
    y: bool,
    a: bool,
    b: bool,
    sl_right: bool,
    sr_right: bool,
    c: bool,
    home: bool,
    stick_rx: f32,
    stick_ry: f32,
    stick_left: bool,
    minus: bool,
    zl: bool,
    l: bool,
    up: bool,
    left: bool,
    right: bool,
    down: bool,
    sl_left: bool,
    sr_left: bool,
    capture: bool,
    stick_lx: f32,
    stick_ly: f32,
    gl: bool,
    gr: bool,

    // Motion Data
    accel_x: f32,
    accel_y: f32,
    accel_z: f32,
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
}

impl ControllerState {
    pub fn apply_to(&self, gamepad: &mut XGamepad) {
        for (button, output) in &[
            (self.up, XButtons!(UP)),
            (self.down, XButtons!(DOWN)),
            (self.left, XButtons!(LEFT)),
            (self.right, XButtons!(RIGHT)),
            (self.minus, XButtons!(BACK)),
            (self.l, XButtons!(LB)),
            (self.stick_left, XButtons!(LTHUMB)),
            (self.a, XButtons!(UP)),
            (self.x, XButtons!(DOWN)),
            (self.b, XButtons!(LEFT)),
            (self.y, XButtons!(RIGHT)),
            (self.plus, XButtons!(BACK)),
            (self.r, XButtons!(LB)),
            (self.stick_right, XButtons!(LTHUMB)),
            (self.home, XButtons!(GUIDE)),
        ] {
            if *button { gamepad.buttons.raw |= output.raw; }
        }
        if self.zl {
            gamepad.left_trigger = u8::MAX;
        }
        if self.zr {
            gamepad.right_trigger = u8::MAX;
        }
        gamepad.thumb_lx = (self.stick_lx * (i16::MAX as f32)).round() as i16;
        gamepad.thumb_ly = (self.stick_ly * (i16::MAX as f32)).round() as i16;
        gamepad.thumb_rx = (self.stick_rx * (i16::MAX as f32)).round() as i16;
        gamepad.thumb_ry = (self.stick_ry * (i16::MAX as f32)).round() as i16;
    }
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
