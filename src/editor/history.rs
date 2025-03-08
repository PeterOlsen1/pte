use super::{
    cursor::Cursor,
    commands::Command,
};
use crate::utils::utils::dbg;
use std::collections::VecDeque;

pub struct History {
    undo_stack: VecDeque<HistoryEntry>,
    redo_stack: VecDeque<HistoryEntry>,
    max_size: usize,
}

#[derive(Clone, Debug)]
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

    pub fn redo_len(&self) -> usize {
        self.redo_stack.len()
    }

    /// Ensure undo stack is within normal size when pushing entries to it
    pub fn push_history(&mut self, entry: HistoryEntry) {
        if self.len() > self.max_size {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(entry);
    }

    /// Ensure redo stack is within normal size when pushing to it
    pub fn push_redo(&mut self, entry: HistoryEntry) {
        if self.redo_len() > self.max_size {
            self.redo_stack.pop_front();
        }
        self.redo_stack.push_back(entry);
    }

    pub fn pop_back(&mut self) -> Option<HistoryEntry> {
        self.undo_stack.pop_back()
    }

    /// Logical driver for the undo method
    /// 
    /// If we have an undo that is a character, keep going
    /// until we no longer placed characters. This way
    /// we can undo whole words at a time.
    pub fn undo(&mut self) -> Option<HistoryEntry> {
        let entry = self.undo_stack.pop_back();
        match entry {
            Some(entry) => {
                //need to keep track of the last entry, it didn't work when i didn't keep track of it
                let mut last_entry = entry.clone();

                //undo entries until we get to a backspace
                self.push_redo(entry.clone());
                while last_entry.command == Command::AddChar {
                    if let Some(entry) = self.undo_stack.pop_back() {
                        last_entry = entry.clone();
                        self.push_redo(entry.clone());
                    } else {
                        return Some(last_entry);
                    }
                }
                Some(last_entry)
            },
            None => None,
        }
    }
    
    /// Logical drivers for the redo function 
    /// 
    /// Same logic as undo: if we redo a character placement, keep going
    /// until we no longer are redoing characters. This way we can
    /// redo entire words at a time
    pub fn redo(&mut self) -> Option<HistoryEntry> {
        let entry = self.redo_stack.pop_back();
        match entry {
            Some(entry) => {
                let mut last_entry = entry.clone();

                //redo entries until we get to a backspace
                self.push_history(entry.clone());
                while last_entry.command == Command::AddChar {
                    if let Some(entry) = self.redo_stack.pop_back() {
                        last_entry = entry.clone();
                        self.push_history(entry.clone());
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