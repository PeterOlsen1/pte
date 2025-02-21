#[derive(Debug, Clone)]
pub struct Cursor {
    pub clipboard: String,
    pub line: u16,
    pub col: u16,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            clipboard: String::new(),
        }
    }
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.col == other.col
    }
}