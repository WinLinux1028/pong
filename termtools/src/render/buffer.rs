use std::ops::{Deref, DerefMut};

use crate::render::{Buffer, TermCell};

impl Buffer {
    pub(crate) fn new(size_x: usize, size_y: usize) -> Self {
        let mut buffer = Vec::with_capacity(size_y);
        for _ in 0..size_y {
            let mut buffer_x = Vec::with_capacity(size_x);
            for _ in 0..size_x {
                buffer_x.push(TermCell::new(' '));
            }
            buffer.push(buffer_x);
        }

        Self(buffer)
    }

    pub fn clean(&mut self) {
        for i in &mut self.0 {
            for j in i {
                *j = TermCell::new(' ');
            }
        }
    }
}

impl Deref for Buffer {
    type Target = Vec<Vec<TermCell>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
