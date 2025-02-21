#[derive(PartialEq)]
pub enum Command {
    NULL,
    GotoLine,
    MoveCursor,
}

impl Command {
    pub fn new() -> Self {
        Self::NULL
    }
}