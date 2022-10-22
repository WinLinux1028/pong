use std::{
    io::stdin,
    sync::{Arc, Mutex},
    thread,
};

use termion::{event, input::TermRead};

use crate::input::{MouseStatus, TermInput};

impl TermInput {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let key = Arc::new(Mutex::new(None));
        let mouse = Arc::new(Mutex::new(None));

        let key2 = Arc::clone(&key);
        let mouse2 = Arc::clone(&mouse);
        // 一度起動するとずっとこのスレッドは残り続けるが...仕方ない
        thread::spawn(move || {
            let key = key2;
            let mouse = mouse2;

            // イベント処理ループ
            for i in stdin().events() {
                let i = match i {
                    Ok(o) => o,
                    Err(_) => continue,
                };

                if let event::Event::Key(i) = i {
                    // キーボード入力の場合
                    *key.lock().unwrap() = Some(i);
                } else if let event::Event::Mouse(i) = i {
                    // マウス入力の場合
                    let mut mouse = mouse.lock().unwrap();
                    if let event::MouseEvent::Press(button, x, y) = i {
                        // クリックされた場合
                        match &*mouse {
                            Some(_) => continue, // 同時に2つ以上のボタンが押された場合は無視
                            None => *mouse = Some(MouseStatus::new(button, x, y)), // ここに来るということは､さっきまではマウスボタンは放されていた
                        }
                    } else if let event::MouseEvent::Hold(x, y) = i {
                        // ドラッグされた場合
                        match &mut *mouse {
                            Some(s) => *s = MouseStatus::new(s.button, x, y),
                            None => continue, // マウスのボタンが押されてないのにドラッグされるのはおかしいため無視
                        }
                    } else {
                        // マウスのボタンが放された場合
                        *mouse = None;
                    }
                }
            }
        });

        Self { key, mouse }
    }

    /// 前回のこの関数の実行時以降の入力を取得する
    pub fn get_input(&self) -> (Option<event::Key>, Option<MouseStatus>) {
        let key = self.key.lock().unwrap().take();

        let mut mlock = self.mouse.lock().unwrap();
        let mouse = if let Some(s) = *mlock {
            // ホイール操作がReleaseされない対策
            match s.button {
                event::MouseButton::WheelUp => mlock.take(),
                event::MouseButton::WheelDown => mlock.take(),
                _ => *mlock,
            }
        } else {
            *mlock
        };

        (key, mouse)
    }
}
