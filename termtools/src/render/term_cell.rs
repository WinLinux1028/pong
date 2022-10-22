use crate::render::{color, Style, TermCell};

impl TermCell {
    #[allow(clippy::new_without_default)]
    pub fn new(char: char) -> Self {
        Self {
            char,
            style: Style::new(),
            bgcolor: Box::new(color::Reset),
            fgcolor: Box::new(color::Reset),
        }
    }
}

impl Clone for TermCell {
    fn clone(&self) -> Self {
        Self {
            char: self.char,
            style: self.style,
            bgcolor: dyn_clone::clone_box(&*self.bgcolor),
            fgcolor: dyn_clone::clone_box(&*self.fgcolor),
        }
    }
}

impl std::fmt::Display for TermCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.style,
            color::Bg(&*self.bgcolor as &dyn color::Color),
            color::Fg(&*self.fgcolor as &dyn color::Color),
            self.char,
        )
    }
}
