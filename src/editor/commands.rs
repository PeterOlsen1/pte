#[derive(PartialEq)]
pub enum Command {
    NULL,
    GOTO_LINE,
    MOVE_CURSOR,
}

impl Command {
    pub fn new() -> Self {
        Self::NULL
    }
}