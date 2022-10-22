use std::f64::consts::PI;

use rand::{rngs::OsRng, Rng};
use termtools::{render::color, Position};

use crate::{
    object::{Ball, Player},
    Game,
};

impl Ball {
    pub fn new(game: &mut Game, leftup: Position) -> Self {
        let window = game.objrend.create_window(leftup, 1, 1);
        let mut buffer = window.lock().unwrap();
        buffer.buffer[0][0].char = '∎';
        buffer.buffer[0][0].fgcolor = Box::new(color::LightWhite);
        drop(buffer);
        Self {
            window,
            angle_rad: PI / 6.0,
            position: (leftup.x as f64, leftup.y as f64),
        }
    }

    pub fn motion(&mut self, player1: &Player, player2: &Player) -> Option<usize> {
        let mut window = self.window.lock().unwrap();

        // ボールの移動を計算
        self.position.0 += (19.0 / 62.5) * f64::cos(self.angle_rad);
        self.position.1 += (19.0 / 62.5) * f64::sin(self.angle_rad);

        if self.position.1 > 23.0 {
            // 下にはみ出した場合
            self.position.1 = 23.0;
            self.angle_rad = -self.angle_rad;
        } else if self.position.1 < 0.0 {
            // 上にはみ出した場合
            self.position.1 = 0.0;
            self.angle_rad = -self.angle_rad;
        }
        window.leftup.y = f64::round(self.position.1) as isize; // 実際にターミナルに表示される位置

        if self.position.0 <= 0.0 {
            // 左にはみ出した場合
            self.position.0 = 0.0;
            // 板に当たったか判定
            let player1 = player1.window.lock().unwrap();
            if player1.leftup.y <= window.leftup.y
                && window.leftup.y < player1.leftup.y + (player1.buffer.len() as isize)
            {
                // 当たったら跳ね返す
                self.angle_rad = 0.0 + OsRng.gen_range((-PI / 3.0)..=(PI / 3.0));
            } else {
                // 外したらプレイヤー2の勝ち
                return Some(2);
            }
        } else if self.position.0 >= 79.0 {
            // 右にはみ出した場合
            self.position.0 = 79.0;
            // 板に当たったか判定
            let player2 = player2.window.lock().unwrap();
            if player2.leftup.y <= window.leftup.y
                && window.leftup.y < player2.leftup.y + (player2.buffer.len() as isize)
            {
                // 当たったら跳ね返す
                self.angle_rad = PI + OsRng.gen_range((-PI / 3.0)..=(PI / 3.0));
            } else {
                // 外したらプレイヤー1の勝ち
                return Some(1);
            }
        }
        window.leftup.x = f64::round(self.position.0) as isize; // 実際にターミナルに表示される位置

        None
    }
}
