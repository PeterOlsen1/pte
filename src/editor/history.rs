use super::{
    cursor::Cursor,
    commands::Command,
};
use std::collections::VecDeque;

pub struct History {
    undo_stack: VecDeque<HistoryEntry>,
    redo_stack: VecDeque<HistoryEntry>,
    max_size: usize,
}

#[derive(Clone)]
pub struct HistoryEntry {
    cursors: Vec<Cursor>,
    lines: Vec<String>,
    command: Command,
}

impl History {
    pub fn new() -> History {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_size: 100,
        }
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
    
    pub fn len(&self) -> usize {
        self.undo_stack.len()
    }

    pub fn push_history(&mut self, entry: HistoryEntry) {
        self.undo_stack.push_back(entry);
    }

    pub fn pop_back(&mut self) -> Option<HistoryEntry> {
        self.undo_stack.pop_back()
    }

    pub fn undo(&mut self) -> Option<HistoryEntry> {
        let entry = self.undo_stack.pop_back();
        match entry {
            Some(entry) => {

                //redo entries until we get to a backspace
                self.redo_stack.push_back(entry.clone());
                while entry.command == Command::AddChar {
                    if let Some(entry) = self.undo_stack.pop_back() {
                        self.redo_stack.push_back(entry.clone());
                    } else {
                        break;
                    }
                }
                Some(entry)
            },
            None => None,
        }
    }
    
    pub fn redo(&mut self) -> Option<HistoryEntry> {
        if let Some(entry) = self.redo_stack.pop_back() {
            self.undo_stack.push_back(entry.clone());
            Some(entry)
        } else {
            None
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