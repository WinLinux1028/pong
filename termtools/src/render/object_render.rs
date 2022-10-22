use std::{
    io::{stderr, stdout},
    sync::{Arc, Mutex},
};

use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

use crate::{
    render::{screen_drawer::ScreenDrawer, Buffer, Window},
    ObjectRender, Position,
};

impl ObjectRender {
    pub fn new() -> Self {
        Self::default()
    }

    /// Windowを生成する
    /// 生成されたWindowは次の描画で表示される
    pub fn create_window(
        &mut self,
        leftup: Position,
        size_x: usize,
        size_y: usize,
    ) -> Arc<Mutex<Window>> {
        let buffer = Buffer::new(size_x, size_y);

        let window = Arc::new(Mutex::new(Window {
            leftup,
            buffer,
            killed: false,
        }));
        self.window.push(Arc::clone(&window));
        window
    }

    /// 描画を実行する
    pub fn rendering(&mut self) {
        self.drawer.buffer.clean();

        let mut errindex = 0; // エラーハンドリング用

        loop {
            // 描画処理､エラーハンドリングを楽にしたいので無名関数にする
            let err = (|objrend: &mut Self, errindex: &mut usize| -> Option<()> {
                // ウィンドウを順に描画
                for i in objrend.window[*errindex..].iter().enumerate() {
                    let i = i.1.lock().ok()?;
                    (!i.killed).then_some(())?; // ウィンドウが閉じられていないか確認

                    // ウィンドウのバッファの開始位置
                    let y_window_start = if i.leftup.y.is_positive() {
                        0
                    } else {
                        i.leftup.y.unsigned_abs()
                    };
                    let x_window_start = if i.leftup.x.is_positive() {
                        0
                    } else {
                        i.leftup.x.unsigned_abs()
                    };
                    // 画面のバッファの開始位置
                    let y_screen_start = if i.leftup.y.is_negative() {
                        0
                    } else {
                        i.leftup.y as usize
                    };
                    let x_screen_start = if i.leftup.x.is_negative() {
                        0
                    } else {
                        i.leftup.x as usize
                    };

                    let window = i.buffer.iter().skip(y_window_start);
                    let screen = objrend.drawer.buffer.iter_mut().skip(y_screen_start);
                    for j in window.zip(screen) {
                        let window = j.0.iter().skip(x_window_start);
                        let screen = j.1.iter_mut().skip(x_screen_start);
                        for k in window.zip(screen) {
                            *k.1 = k.0.clone();
                        }
                    }

                    *errindex += 1;
                }
                Some(())
            })(self, &mut errindex);

            //エラーハンドリング
            match err {
                Some(_) => break,
                None => {
                    self.window.remove(errindex); // 失敗していたらそのウィンドウを強制終了し､描画を再開
                }
            }
        }

        self.drawer.update_screen();
    }

    /// trueならマウス操作をプログラム自身が操作できるようにする
    /// ただし､この状態ではターミナル側の右クリックメニューや選択機能が使えなくなるので､falseで無効化することもできる
    pub fn mouse(&mut self, state: bool) {
        self.drawer.screen = Box::new(stdout()); // 一旦self.drawer.screenの内容をdropする
        let base = AlternateScreen::from(stderr().lock())
            .into_raw_mode()
            .unwrap();
        if state {
            self.drawer.screen = Box::new(MouseTerminal::from(base));
        } else {
            self.drawer.screen = Box::new(base);
        }
    }
}

impl Default for ObjectRender {
    fn default() -> Self {
        Self {
            window: Vec::new(),
            drawer: ScreenDrawer::new(),
        }
    }
}

impl Drop for ObjectRender {
    fn drop(&mut self) {
        self.mouse(false);
    }
}
