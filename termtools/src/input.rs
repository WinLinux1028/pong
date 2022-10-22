use std::sync::{Arc, Mutex};

pub use termion::event;

use crate::Position;

mod term_input;
pub struct TermInput {
    key: Arc<Mutex<Option<event::Key>>>,
    mouse: Arc<Mutex<Option<MouseStatus>>>,
}

mod mouse_status;
#[derive(Clone, Copy)]
pub struct MouseStatus {
    pub button: event::MouseButton,
    pub position: Position,
}
