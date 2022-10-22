pub mod object;

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use termtools::{ObjectRender, Position, TermInput};

use crate::object::{Ball, Choices, Player, Text, Title};

fn main() {
    let mut game = Game::new();
    game.objrend.mouse(true);
    let input = TermInput::new();

    loop {
        let start = Instant::now();

        let result = game.next_frame(&input);
        if result.is_none() {
            break;
        }
        game.objrend.rendering();

        if result.unwrap() {
            sleep(Duration::from_secs_f64(5.0));
            break;
        }

        let stop = start.elapsed().as_secs_f64();
        let cooldown = 1.0 / 62.50 - stop;
        if cooldown.is_sign_positive() {
            sleep(Duration::from_secs_f64(cooldown));
        }
    }
}

pub struct Game {
    pub objrend: ObjectRender,
    title: Option<Title>,
    modeselect: Option<Choices>,
    player1: Option<Player>,
    player2: Option<Player>,
    ball: Option<Ball>,
    dialog: Option<Text>,
}

impl Game {
    fn new() -> Game {
        let objrend = ObjectRender::new();
        let mut game = Game {
            objrend,
            title: None,
            modeselect: None,
            player1: None,
            player2: None,
            ball: None,
            dialog: None,
        };
        game.to_title();

        game
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_title(&mut self) {
        self.title = Some(Title::new(self, Position::new(25, 2)));
        self.modeselect = Choices::new(self, Position::new(33, 15), "vs myself\nexit");

        self.player1 = None;
        self.player2 = None;
    }

    fn next_frame(&mut self, input: &TermInput) -> Option<bool> {
        let (key, mouse) = input.get_input();
        if let Some(modeselect) = &mut self.modeselect {
            let result = modeselect.control(&key, &mouse);
            if let Some(result) = result {
                self.title = None;
                self.modeselect = None;

                self.ball = Some(Ball::new(self, Position::new(39, 11)));
                self.player1 = Some(Player::new(self, Position::new(0, 9), 1, false));
                if result == 0 {
                    self.player2 = Some(Player::new(self, Position::new(79, 9), 1, false));
                } else {
                    return None;
                }
            }
        } else {
            self.player1.as_mut().unwrap().control(&key);
            self.player2.as_mut().unwrap().control(&key);
            let result = self.ball.as_mut().unwrap().motion(
                self.player1.as_mut().unwrap(),
                self.player2.as_mut().unwrap(),
            );
            if let Some(result) = result {
                self.dialog = Some(Text::new(
                    self,
                    Position::new(33, 11),
                    &format!("Player {} WIN!", result),
                ));
                return Some(true);
            }
        }
        Some(false)
    }
}
