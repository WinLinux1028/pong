use termtools::{
    input::event::{self, Key},
    render::color,
    Position,
};

use crate::{object::Player, Game};

impl Player {
    pub fn new(game: &mut Game, leftup: Position, player_number: usize, bot: bool) -> Self {
        let window = game.objrend.create_window(leftup, 1, 4);
        for i in &mut *window.lock().unwrap().buffer {
            i[0].bgcolor = Box::new(color::White);
        }
        Self {
            window,
            player_number,
            bot,
        }
    }

    #[allow(clippy::if_same_then_else)]
    pub fn control(&mut self, key: &Option<Key>) {
        let mut window = self.window.lock().unwrap();
        if self.bot {
            // stub
        } else if let Some(key) = key {
            if self.player_number == 1 {
                match key {
                    event::Key::Char('w') => {
                        window.leftup.y -= 2;
                    }
                    event::Key::Char('s') => {
                        window.leftup.y += 2;
                    }
                    _ => {}
                }
            } else {
                //stub
            }
        }

        if window.leftup.y > 20 {
            window.leftup.y = 20;
        } else if window.leftup.y < 0 {
            window.leftup.y = 0;
        }
    }
}
