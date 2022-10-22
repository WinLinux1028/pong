pub mod object;

use std::time::Instant;
use termtools::{ObjectRender, Position};

use crate::object::example::Example;

fn main() {
    let mut objrend = ObjectRender::new();
    let obj = Example::new(&mut objrend, Position::new(0, 0));
    objrend.rendering();
    loop {
        let start = Instant::now();
        objrend.rendering();
        let mut objwin = obj.window.lock().unwrap();
        objwin.leftup.y += 1;
        if objwin.leftup.y >= 24 {
            objwin.leftup.y = 0;
            drop(objwin);
            let stop = start.elapsed().as_secs_f64();
            println!("{}s, {}fps\r", stop, 1.0 / stop);
        }
    }
}
