#![feature(trait_upcasting)]

pub mod render;
pub use crate::render::ObjectRender;

pub mod input;
pub use crate::input::TermInput;

/// ターミナル上の座標｡ (0, 0)は左上を指す
#[derive(Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }
}
