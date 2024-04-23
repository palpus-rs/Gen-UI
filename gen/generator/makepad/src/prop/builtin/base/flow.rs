use std::default;

#[derive(Debug,Clone, Copy,Default)]
pub enum Flow {
    #[default]
    Right,
    Down,
    //Left,
    //Up,
    Overlay,
    RightWrap,
}
