pub enum ControllerState {
    Left(LeftJoyConState),
    Right(RightJoyConState),
}

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
}

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
}
