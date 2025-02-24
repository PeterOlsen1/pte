#[derive(Debug, Clone)]
pub struct Cursor {
    pub clipboard: String,
    pub line: u16,
    pub col: u16,
    pub selection: ((u16, u16), (u16, u16)),
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            clipboard: String::new(),
            selection: ((0, 0), (0, 0)),
        }
    }

    pub fn expand_selection(&mut self, line: u16, col: u16) {
        if self.selection.0 == (0, 0) {
            self.selection.0 = (self.line, self.col);
        }
        self.selection.1 = (line, col);
    }

}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.col == other.col
    }
}