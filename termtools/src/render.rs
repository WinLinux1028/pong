use std::sync::{Arc, Mutex};

use dyn_clone::DynClone;

use crate::{render::screen_drawer::ScreenDrawer, Position};

pub use termion::color;
pub(crate) mod screen_drawer;

mod term_cell;
/// ターミナルの1マスの情報をまとめたもの
pub struct TermCell {
    pub char: char,
    pub style: Style,
    pub bgcolor: Box<dyn Color>,
    pub fgcolor: Box<dyn Color>,
}

mod style;
/// ターミナル上の文字の装飾
#[derive(Clone, Copy, Default)]
pub struct Style {
    /// 点滅
    pub blink: bool,
    /// 太字
    pub bold: bool,
    /// イタリック体
    pub italic: bool,
    /// 下線
    pub underline: bool,
    /// 取り消し線
    pub crossed_out: bool,
    /// 四角を重ねる
    pub framed: bool,
}

mod buffer;
/// 画面やウィンドウのバッファ
pub struct Buffer(Vec<Vec<TermCell>>);

mod window;
/// 画面の内容を構成する大きなまとまり
pub struct Window {
    /// ウィンドウの左上の場所
    pub leftup: Position,
    /// ウィンドウに表示したい内容を書き込むバッファ
    pub buffer: Buffer,
    killed: bool,
}

mod object_render;
/// Windowを画面に描画するオブジェクト
/// まず最初にこれをnew()する
pub struct ObjectRender {
    window: Vec<Arc<Mutex<Window>>>,
    drawer: ScreenDrawer,
}

/// クローン可能なColorトレイト
pub trait Color: color::Color + DynClone {}
impl<T: color::Color + Clone> Color for T {}
