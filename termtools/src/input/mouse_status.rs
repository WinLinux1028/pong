use termion::event::MouseButton;

use crate::{input::MouseStatus, Position};

impl MouseStatus {
    pub(crate) fn new(button: MouseButton, x: u16, y: u16) -> Self {
        Self {
            button,
            position: Position::new((x - 1) as isize, (y - 1) as isize), // termionでは(1, 1)が原点なので(0, 0)が原点になるよう直す
        }
    }

    /// ウィンドウ内でのカーソルの位置を取得する
    pub fn pos_in_window(
        &self,
        mut leftup: Position,
        size_x: usize,
        size_y: usize,
    ) -> Option<Position> {
        leftup.x = self.position.x - leftup.x;
        leftup.y = self.position.y - leftup.y;
        if leftup.x.is_negative() || leftup.y.is_negative() {
            return None;
        }
        if leftup.x > (size_x as isize) - 1 || leftup.y > (size_y as isize) - 1 {
            return None;
        }
        Some(leftup)
    }
}
