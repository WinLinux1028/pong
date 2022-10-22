use std::{
    fmt::Write as _,
    io::{stderr, Write},
    ops::Drop,
};

use termion::{cursor, raw::IntoRawMode, screen::AlternateScreen};

use crate::render::Buffer;

/// バッファの内容を画面に描画するオブジェクト
pub struct ScreenDrawer {
    pub buffer: Buffer, // (y, x)座標､(0, 0)は左上を表す
    framebuf: String,
    pub screen: Box<dyn Write>,
}

impl ScreenDrawer {
    pub fn new() -> Self {
        // バッファを準備(80x24)
        let buffer = Buffer::new(80, 24);

        // いちいちシステムコール呼ばれては遅いのでバッファリングする
        let framebuf = String::new();

        // 描写先を準備､バッファリングされないようstderrを使う
        let mut screen = AlternateScreen::from(stderr().lock())
            .into_raw_mode()
            .unwrap(); // 画面を切り替え､rawモードに切り替える
        write!(&mut screen, "{}", cursor::Hide).unwrap(); // カーソルを隠し､カーソルを左上に移動
        screen.flush().unwrap();

        Self {
            buffer,
            framebuf,
            screen: Box::new(screen),
        }
    }

    /// バッファの内容を出力する
    pub fn update_screen(&mut self) {
        self.framebuf.clear();
        let _ = write!(&mut self.framebuf, "{}", cursor::Goto(1, 1)); // 左上にカーソルをセット

        for i in &*self.buffer {
            for j in i {
                let _ = write!(&mut self.framebuf, "{}", j);
            }
            let _ = write!(&mut self.framebuf, "\r{}", cursor::Down(1)); // 下の行の1文字目に移動
        }
        let _ = self.screen.write_all(self.framebuf.as_bytes());
    }
}

impl Drop for ScreenDrawer {
    fn drop(&mut self) {
        eprint!("{}", cursor::Show);
    }
}
