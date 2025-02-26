use super::{
    cursor::Cursor,
    commands::Command,
};
use std::collections::VecDeque;

struct History {
    undo_stack: VecDeque<HistoryEntry>,
    redo_stack: VecDeque<HistoryEntry>,
}

struct HistoryEntry {
    cursors: Vec<Cursor>,
    lines: Vec<String>,
    command: Command,
}

impl History {
    pub fn new() -> History {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }
}

impl HistoryEntry {
    pub fn new() -> HistoryEntry {
        Self {
            cursors: Vec::new(),
            lines: Vec::new(),
            command: Command::NULL,
        }
    }

    pub fn from(cursors: Vec<Cursor>, lines: Vec<String>, command: Command) -> HistoryEntry {
        Self {
            cursors,
            lines,
            command,
        }
    }
}