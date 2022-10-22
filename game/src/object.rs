use std::sync::{Arc, Mutex};

use termtools::render::Window;

mod text;
pub struct Text {
    pub window: Arc<Mutex<Window>>,
}

mod title;
pub struct Title {
    pub inner: Text,
}

mod choices;
pub struct Choices {
    pub window: Text,
    choose: usize,
}

mod player;
pub struct Player {
    pub window: Arc<Mutex<Window>>,
    player_number: usize,
    bot: bool,
}

mod ball;
pub struct Ball {
    pub window: Arc<Mutex<Window>>,
    angle_rad: f64,
    position: (f64, f64),
}
