use termtools::{
    input::{event, MouseStatus},
    render::color,
    Position,
};

use crate::{
    object::{Choices, Text},
    Game,
};

impl Choices {
    pub fn new(game: &mut Game, leftup: Position, choices: &str) -> Option<Self> {
        let window = Text::new(game, leftup, choices);

        Some(Self { window, choose: 0 })
    }

    pub fn control(
        &mut self,
        key: &Option<event::Key>,
        mouse: &Option<MouseStatus>,
    ) -> Option<usize> {
        let choose_backup = self.choose;
        let mut window = self.window.window.lock().unwrap();

        if let Some(mouse) = mouse {
            //マウス操作判定
            if let event::MouseButton::Left = mouse.button {
                // 左ボタンでクリックした場合
                let mouse_pos =
                    mouse.pos_in_window(window.leftup, window.buffer[0].len(), window.buffer.len());
                if let Some(mouse_pos) = mouse_pos {
                    // ウィンドウ内の場所をクリックしていた場合
                    return Some(mouse_pos.y as usize);
                }
            }
        } else if let Some(key) = key {
            match key {
                event::Key::Up => {
                    self.choose = if self.choose == 0 {
                        window.buffer.len() - 1
                    } else {
                        self.choose - 1
                    };
                }
                event::Key::Down => {
                    self.choose = if self.choose == window.buffer.len() - 1 {
                        0
                    } else {
                        self.choose + 1
                    };
                }
                event::Key::Char('\n') => return Some(self.choose),
                _ => {}
            }
        }

        #[allow(clippy::significant_drop_in_scrutinee)]
        for i in &mut window.buffer[self.choose] {
            i.bgcolor = Box::new(color::White);
            i.fgcolor = Box::new(color::Black);
        }

        if choose_backup != self.choose {
            for i in &mut window.buffer[choose_backup] {
                i.bgcolor = Box::new(color::Reset);
                i.fgcolor = Box::new(color::Reset);
            }
        }

        None
    }
}
