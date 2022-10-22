use std::sync::{Arc, Mutex};

use termtools::{
    render::{color, TermCell, Window},
    ObjectRender, Position,
};

const OUTPUT: &str = "Hello, World!";

pub struct Example {
    pub window: Arc<Mutex<Window>>,
}

impl Example {
    pub fn new(objrend: &mut ObjectRender, leftup: Position) -> Example {
        let output: Vec<char> = OUTPUT.chars().collect();
        let window = objrend.create_window(leftup, output.len(), 1);

        for i in output
            .into_iter()
            .zip(window.lock().unwrap().buffer[0].iter_mut())
        {
            *i.1 = TermCell::new(i.0);
            i.1.fgcolor = Box::new(color::Rgb(255, 255, 0));
            i.1.bgcolor = Box::new(color::Rgb(255, 0, 0));
            i.1.style.blink = true;
            i.1.style.bold = true;
            i.1.style.italic = true;
            i.1.style.underline = true;
            i.1.style.bold = true;
            i.1.style.crossed_out = true;
            i.1.style.framed = true;
        }

        Example { window }
    }
}

impl Drop for Example {
    fn drop(&mut self) {
        self.window.lock().unwrap().kill()
    }
}
