use btleplug::api::ValueNotification;

pub enum ControllerState {
    Left(LeftJoyConState),
    Right(RightJoyConState),
}

#[derive(Debug)]
pub struct LeftJoyConState {
    stick: bool,
    minus: bool,
    zl: bool,
    l: bool,
    up: bool,
    left: bool,
    right: bool,
    down: bool,
    sl: bool,
    sr: bool,
    capture: bool,
    stick_x: f32,
    stick_y: f32,
}

#[derive(Debug)]
pub struct RightJoyConState {
    stick: bool,
    plus: bool,
    zr: bool,
    r: bool,
    x: bool,
    y: bool,
    a: bool,
    b: bool,
    sl: bool,
    sr: bool,
    c: bool,
    home: bool,
    stick_x: f32,
    stick_y: f32,
}

impl From<ValueNotification> for LeftJoyConState {
    fn from(notification: ValueNotification) -> Self {
        let buttons = &notification.value[0x02..0x04];
        let (x, y) = decode_stick_data(&notification.value[0x5..0x8]);
        LeftJoyConState {
            stick: (buttons[0] & 0x80) > 0,
            minus: (buttons[0] & 0x40) > 0,
            zl: (buttons[0] & 0x20) > 0,
            l: (buttons[0] & 0x10) > 0,
            up: (buttons[0] & 0x08) > 0,
            left: (buttons[0] & 0x04) > 0,
            right: (buttons[0] & 0x02) > 0,
            down: (buttons[0] & 0x01) > 0,
            sl: (buttons[1] & 0x80) > 0,
            sr: (buttons[1] & 0x40) > 0,
            capture: (buttons[1] & 0x01) > 0,

            stick_x: x,
            stick_y: y,
        }
    }
}

impl From<ValueNotification> for RightJoyConState {
    fn from(notification: ValueNotification) -> Self {
        let buttons = &notification.value[0x02..0x04];
        let (x, y) = decode_stick_data(&notification.value[0x5..0x8]);
        RightJoyConState {
            stick: (buttons[0] & 0x80) > 0,
            plus: (buttons[0] & 0x40) > 0,
            zr: (buttons[0] & 0x20) > 0,
            r: (buttons[0] & 0x10) > 0,
            x: (buttons[0] & 0x08) > 0,
            y: (buttons[0] & 0x04) > 0,
            a: (buttons[0] & 0x02) > 0,
            b: (buttons[0] & 0x01) > 0,
            sl: (buttons[1] & 0x80) > 0,
            sr: (buttons[1] & 0x40) > 0,
            home: (buttons[1] & 0x01) > 0,
            c: (buttons[1] & 0x10) > 0,

            stick_x: x,
            stick_y: y,
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
