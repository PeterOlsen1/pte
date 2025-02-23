#[derive(PartialEq)]
pub enum Command {
    NULL,
    GotoLine,
    MoveCursor,
    Find,
    FindSelection,
    OpenFile,
    AddCharater,
    DeleteCharacter,
    SavePrompt
}

impl Command {
    pub fn new() -> Self {
        Self::NULL
    }
}