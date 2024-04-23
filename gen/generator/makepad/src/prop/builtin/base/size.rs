use std::default;

#[derive(Debug,Clone,Default)]

pub enum Size {
    #[default]
    Fill,
    Fixed(f64),
    Fit,
    All
}