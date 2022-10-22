use termion::style;

use crate::render::Style;

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut last = write!(f, "{}", style::Reset);
        if self.blink {
            last = write!(f, "{}", style::Blink);
        }
        if self.bold {
            last = write!(f, "{}", style::Bold);
        }
        if self.italic {
            last = write!(f, "{}", style::Italic);
        }
        if self.underline {
            last = write!(f, "{}", style::Underline);
        }
        if self.crossed_out {
            last = write!(f, "{}", style::CrossedOut);
        }
        if self.framed {
            last = write!(f, "{}", style::Framed);
        }
        last
    }
}
