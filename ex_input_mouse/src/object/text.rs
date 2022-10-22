use std::sync::{Arc, Mutex};

use termtools::{
    render::{TermCell, Window},
    ObjectRender, Position,
};

pub struct Text {
    pub window: Arc<Mutex<Window>>,
}

impl Text {
    pub fn new(objrend: &mut ObjectRender, output: &str, leftup: Position) -> Text {
        let output: Vec<Vec<char>> = output.split('\n').map(|i| i.chars().collect()).collect();
        let x_max = output.iter().map(|i| i.len()).max().unwrap();
        let window = objrend.create_window(leftup, x_max, output.len());

        for i in output
            .into_iter()
            .zip(window.lock().unwrap().buffer.iter_mut())
        {
            for j in i.0.into_iter().zip(i.1.iter_mut()) {
                *j.1 = TermCell::new(j.0);
            }
        }

        Text { window }
    }
}

impl Drop for Text {
    fn drop(&mut self) {
        self.window.lock().unwrap().kill()
    }
}
