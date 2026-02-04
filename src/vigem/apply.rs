use vigem_client::{XButtons, XGamepad};

use crate::bluetooth::state::ControllerState;

pub trait Apply {
    fn apply(&mut self, gamepad: ControllerState);
}

impl Apply for XGamepad {
    fn apply(&mut self, state: ControllerState) {
        for (button, output) in [
            // left
            (state.up, XButtons!(UP)),
            (state.down, XButtons!(DOWN)),
            (state.left, XButtons!(LEFT)),
            (state.right, XButtons!(RIGHT)),
            (state.minus, XButtons!(BACK)),
            (state.l, XButtons!(LB)),
            (state.stick_left, XButtons!(LTHUMB)),
            // right
            (state.a, XButtons!(B)),
            (state.b, XButtons!(A)),
            (state.x, XButtons!(Y)),
            (state.y, XButtons!(X)),
            (state.plus, XButtons!(START)),
            (state.r, XButtons!(RB)),
            (state.home, XButtons!(GUIDE)),
            (state.stick_right, XButtons!(RTHUMB)),
        ] {
            if button {
                self.buttons.raw |= output.raw;
            }
        }

        if state.zl {
            self.left_trigger = u8::MAX;
        }
        if state.zr {
            self.right_trigger = u8::MAX;
        }
        self.thumb_lx = self.thumb_lx.saturating_add((state.stick_lx * (i16::MAX as f32)).round() as i16);
        self.thumb_ly = self.thumb_ly.saturating_add((state.stick_ly * (i16::MAX as f32)).round() as i16);
        self.thumb_rx = self.thumb_rx.saturating_add((state.stick_rx * (i16::MAX as f32)).round() as i16);
        self.thumb_ry = self.thumb_ry.saturating_add((state.stick_ry * (i16::MAX as f32)).round() as i16);

    }
}
