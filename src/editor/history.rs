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
    pub cursors: Vec<Cursor>,
    pub lines: Vec<String>,
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
                //need to keep track of the last entry, it didn't work when i didn't keep track of it
                let mut last_entry = entry.clone();

                //undo entries until we get to a backspace
                self.redo_stack.push_back(entry.clone());
                while entry.command == Command::AddChar {
                    if let Some(entry) = self.undo_stack.pop_back() {
                        last_entry = entry.clone();
                        self.redo_stack.push_back(entry.clone());
                    } else {
                        self.redo_stack.push_back(entry.clone());
                        return Some(last_entry);
                    }
                }
                Some(last_entry)
            },
            None => None,
        }
    }
    
    pub fn redo(&mut self) -> Option<HistoryEntry> {
        let entry = self.redo_stack.pop_back();
        match entry {
            Some(entry) => {
                let mut last_entry = entry.clone();

                //redo entries until we get to a backspace
                self.undo_stack.push_back(entry.clone());
                while entry.command == Command::AddChar {
                    if let Some(entry) = self.redo_stack.pop_back() {
                        last_entry = entry.clone();
                        println!("redoing entry");
                        self.undo_stack.push_back(entry.clone());
                    } else {
                        return Some(last_entry);
                    }
                }
                Some(last_entry)
            }
            None => None,
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