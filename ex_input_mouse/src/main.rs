pub mod object;

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use termtools::{input::event, ObjectRender, Position, TermInput};

use crate::object::text::Text;

fn main() {
    let mut game = Game::new();
    game.objrend.mouse(true);
    let input = TermInput::new();
    loop {
        let start = Instant::now();
        let (key, mouse) = input.get_input();
        if let Some(s) = key {
            let mut window = game.stub.window.lock().unwrap();
            match s {
                event::Key::Up => window.leftup.y -= 1,
                event::Key::Down => window.leftup.y += 1,
                event::Key::Left => window.leftup.x -= 1,
                event::Key::Right => window.leftup.x += 1,
                event::Key::Char(c) => match c {
                    '1' => window.rotate_90(),
                    '2' => window.rotate_180(),
                    '3' => window.rotate_270(),
                    _ => {}
                },
                _ => {}
            }
        }
        if let Some(s) = mouse {
            let mut window = game.stub.window.lock().unwrap();
            window.leftup.x = s.position.x;
            window.leftup.y = s.position.y;
        }
        game.objrend.rendering();

        let stop = start.elapsed().as_secs_f64();
        let cooldown = 1.0 / 62.50 - stop;
        if cooldown.is_sign_positive() {
            sleep(Duration::from_secs_f64(cooldown));
        }
    }
}

struct Game {
    objrend: ObjectRender,
    stub: Text,
}

impl Game {
    fn new() -> Game {
        let mut objrend = ObjectRender::new();
        let stub = Text::new(&mut objrend, "Heeellooo,\nWorld!", Position::new(0, 0));
        Game { objrend, stub }
    }
}
