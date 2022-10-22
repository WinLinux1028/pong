use std::ptr::swap;

use crate::render::{Buffer, TermCell, Window};

impl Window {
    /// ウィンドウを閉じる
    pub fn kill(&mut self) {
        self.killed = true;
    }

    /// ウィンドウの大きさを変える
    /// 大きくなる場合はスペースで埋められ､小さくなる場合は切り捨てられる
    pub fn resize(&mut self, size_x: usize, size_y: usize) {
        self.buffer.resize(size_y, Vec::new());
        for i in &mut *self.buffer {
            i.resize(size_x, TermCell::new(' '))
        }
    }

    /// 90°回転
    pub fn rotate_90(&mut self) {
        self.upside_down();
        self.transpose();
    }

    /// 180°回転
    pub fn rotate_180(&mut self) {
        self.upside_down();
        self.flip_horizontal();
    }

    /// 270°回転
    pub fn rotate_270(&mut self) {
        self.transpose();
        self.upside_down();
    }

    /// 上下反転する
    pub fn upside_down(&mut self) {
        let len = self.buffer.len();
        let half = len / 2;
        for (i, j) in (0..half).zip((0..len).rev()) {
            unsafe {
                swap::<Vec<TermCell>>(&mut self.buffer[i] as *mut _, &mut self.buffer[j] as *mut _);
            }
        }
    }

    /// 左右反転する
    pub fn flip_horizontal(&mut self) {
        for i in &mut *self.buffer {
            let len = i.len();
            let half = len / 2;

            for (j, k) in (0..half).zip((0..len).rev()) {
                unsafe {
                    swap::<TermCell>(&mut i[j] as *mut _, &mut i[k] as *mut _);
                }
            }
        }
    }

    /// バッファを転置する
    fn transpose(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        let size_x = self.buffer[0].len();
        let size_y = self.buffer.len();
        let mut newbuf = Buffer::new(size_y, size_x);

        for i in 0..size_y {
            for j in 0..size_x {
                newbuf[j][i] = self.buffer[i][j].clone();
            }
        }
        self.buffer = newbuf;
    }
}
