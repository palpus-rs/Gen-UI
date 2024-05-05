#[allow(unused_imports)]
use std::default;

#[derive(Clone, Copy, Debug,Default)]
pub enum Role {
    If,
    For,
    #[default]
    Normal
}