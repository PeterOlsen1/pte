#[derive(PartialEq, Clone)]
pub enum Command {
    NULL,
    GotoLine,
    MoveCursor,
    Find,
    Tab,
    FindSelection,
    OpenFile,
    AddCharater,
    Backspace,
    SavePrompt,
    AddChar,
    AddNewLine,
    AddString(String),
}

impl Command {
    pub fn new() -> Self {
        Self::NULL
    }
}