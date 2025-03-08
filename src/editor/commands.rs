#[derive(PartialEq, Clone, Debug)]
pub enum Command {
    NULL,
    GotoLine,
    MoveCursor,
    Find,
    Tab,
    FindSelection,
    OpenFile,
    Backspace,
    Space,
    SavePrompt,
    AddChar,
    AddNewLine,
    Undo,
}

impl Command {
    pub fn new() -> Self {
        Self::NULL
    }
}