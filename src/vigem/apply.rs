pub trait ApplyTo<T> {
    fn apply_to(&self, gamepad: XGamepad);
}

impl ApplyTo for XGamepad {
    fn apply_to(&self, gamepad: XGamepad) {
        for (button, output) in &[
            (self.up, XButtons!(UP)),
            (self.down, XButtons!(DOWN)),
            (self.left, XButtons!(LEFT)),
            (self.right, XButtons!(RIGHT)),
            (self.minus, XButtons!(BACK)),
            (self.l, XButtons!(LB)),
            (self.stick, XButtons!(LTHUMB)),
        ] {
            if *button {
                gamepad.buttons.raw |= output.raw;
            }
        }
        if self.zl {
            gamepad.left_trigger = u8::MAX;
        }
        gamepad.thumb_lx = (self.stick_x * (i16::MAX as f32)).round() as i16;
        gamepad.thumb_ly = (self.stick_y * (i16::MAX as f32)).round() as i16;
    }
}

impl ApplyTo for RightJoyConState {
    fn apply_to(&self, gamepad: &mut XGamepad) {
        for (button, output) in &[
            (self.a, XButtons!(B)),
            (self.b, XButtons!(A)),
            (self.x, XButtons!(Y)),
            (self.y, XButtons!(X)),
            (self.plus, XButtons!(START)),
            (self.r, XButtons!(RB)),
            (self.home, XButtons!(GUIDE)),
            (self.stick, XButtons!(RTHUMB)),
        ] {
            if *button {
                gamepad.buttons.raw |= output.raw;
            }
        }
        if self.zr {
            gamepad.right_trigger = u8::MAX;
        }
        gamepad.thumb_rx = (self.stick_x * (i16::MAX as f32)).round() as i16;
        gamepad.thumb_ry = (self.stick_y * (i16::MAX as f32)).round() as i16;
    }
}
