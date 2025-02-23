#[derive(PartialEq)]
pub enum Command {
    NULL,
    GotoLine,
    MoveCursor,
    Find,
    AddCharater,
    DeleteCharacter,
}

impl Command {
    pub fn new() -> Self {
        Self::NULL
    }
}