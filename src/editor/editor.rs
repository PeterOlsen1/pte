use std::collections::VecDeque;

use super::{
    cursor::Cursor,
    commands::Command,
    finder::Finder
};

macro_rules! edbg {
    ($editor:expr, $(string:expr)*) => {
        $(
            editor.lines.insert(0, $string.to_string());
        )*
    };
}

pub struct Editor {
    pub text: String,
    pub lines: Vec<String>,
    pub cursors: Vec<Cursor>,
    pub filename: String,
    pub file_to_open: String,
    pub history: VecDeque<Vec<String>>,
    pub notif_text: String,
    pub command_mode: bool,
    pub command: Command,
    pub changes_saved: bool,
    pub finder: Finder
}

impl Editor {
    pub fn new() -> Self {
        let mut temp = Self {
            text: String::new(),
            lines: Vec::new(),
            cursors: Vec::new(),
            filename: String::new(),
            file_to_open: String::new(),
            history: VecDeque::new(),
            notif_text: String::from("Editor mode"),
            command_mode: false,
            command: Command::new(),
            changes_saved: true,
            finder: Finder::new()
        };

        temp.cursors.push(Cursor::new());
        temp
    }

    pub fn dbg(&mut self, string: String) {
        self.lines.insert(0, string);
    }
    
    pub fn push_history(&mut self) {
        if (self.history.len() as u16) > 100 {
            self.history.pop_front();
        }

        let mut temp = Vec::new();
        for line in &self.lines {
            temp.push(line.clone());
        }

        self.history.push_back(temp);
    }

    pub fn undo(&mut self) {
        let lines = self.history.pop_back();
        match lines {
            Some(lines) => {
                self.lines = lines;
            }
            None => {
                self.notif_text = String::from("No edits to undo!");
            }
        }
    }
    /**
     * Get a mutable reference to the line we want to work on
     */
    pub fn get_line(&mut self, cursor: &mut Cursor) -> &mut String {
        &mut self.lines[cursor.line as usize]
    }

    /**
     * Delete previous character. If we are at the beginning of the line, 
     * append current line to the last
     */
    pub fn backspace(&mut self) {
        let mut edited_flag = true;
        self.push_history();

        for cursor in &mut self.cursors {
            let col = cursor.col as usize;
    
            if col > 0 {
                // if col > 4 && &self.lines[cursor.line as usize][col - 4..col] == "    " {
                //     let line = &mut self.lines[cursor.line as usize];
                //     line.remove(col - 4);
                //     line.remove(col - 3);
                //     line.remove(col - 2);
                //     line.remove(col - 1);
                //     cursor.col -= 4;
                // } 
                // else {
                    let line = &mut self.lines[cursor.line as usize];
                    line.remove(col as usize - 1);
                    cursor.col -= 1;
                // }
            } 
            else if cursor.line == 0 {
                edited_flag = false;
                continue;
            } 
            else if cursor.line > 0 {
                let prev_len = self.lines[cursor.line as usize - 1].len();
                let line = self.lines[cursor.line as usize].clone();
    
                self.lines[cursor.line as usize - 1].push_str(&line);
                self.lines.remove(cursor.line as usize);
                cursor.line -= 1;
                cursor.col = prev_len as u16;
            }
        }

        if !edited_flag {
            self.history.pop_back();
        }
    }

    pub fn backspace_word(&mut self) {
        let mut edited_flag = true;
        self.push_history();

        //iterate over all cursors
        for cursor in &mut self.cursors {
            let line = &mut self.lines[cursor.line as usize];
            let chars: Vec<char> = line.chars().collect();

            //remove 1 if it is a space
            if chars[cursor.col as usize - 1] == ' ' {
                line.remove(cursor.col as usize - 1);
                cursor.col -= 1;
                edited_flag = true;
                continue;
            }
            
            //remove all characters until a space
            while chars[cursor.col as usize - 1] != ' ' && cursor.col > 0 {
                line.remove(cursor.col as usize - 1);
                cursor.col -= 1;
                edited_flag = true;
            }
        }

        if !edited_flag {
            self.history.pop_back();
        }
    }

    pub fn backspace_line(&mut self) {
        self.push_history();

        for cursor in &mut self.cursors {
            if cursor.col > 0 {
                let line = &mut self.lines[cursor.line as usize];
                line.clear();
                cursor.col = 0;
            }
        }
    }

    pub fn insert(&mut self, c: char) {
        self.push_history();

        for cursor in &mut self.cursors {
            let cursor_x = cursor.col as usize;
            let line = &mut self.lines[cursor.line as usize];
            line.insert(cursor_x, c);
            cursor.col += 1;
        }
    }

    pub fn insert_string(&mut self, s: String) {
        for cursor in &mut self.cursors {
            let cursor_x = cursor.col as usize;
            let line = &mut self.lines[cursor.line as usize];
            line.insert_str(cursor_x, s.as_str());
            cursor.col += s.len() as u16;
        }
    }

    pub fn tab(&mut self) {
        for cursor in &mut self.cursors {
            let col = cursor.col as usize;
            let mut tabs = 0;

            //auto tab if previous line was tabbed in
            if cursor.line > 1 {
                let prev_line = self.lines[cursor.line as usize - 1].clone();
                let chars: Vec<char> = prev_line.chars().collect();
                let mut i = 0;

                while i < chars.len() && chars[i] == ' ' {
                    i += 1;
                }

                tabs += i / 4;
            }

            let line = &mut self.lines[cursor.line as usize];

            for _ in 0..tabs {
                line.insert_str(col, "    ");
                cursor.col += 4;
            }
        }
    }
    pub fn new_line(&mut self) {
        self.push_history();
        
        for cursor in &mut self.cursors {
            let cursor_x = cursor.col as usize;
            let line = &mut self.lines[cursor.line as usize];
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            while i < chars.len() && chars[i] == ' ' {
                i += 1;
            }
            let tabs = i / 4;

            let new_line = &mut line.split_off(cursor_x);
            for _ in 0..tabs {
                new_line.push_str("    ");
            }
            self.lines.insert(cursor.line as usize + 1, new_line.to_owned());
            cursor.line += 1;
            cursor.col = 0;
        }
    }


    //=================================================================================================
    // CURSOR MOVING FUNCTIONS

    pub fn right(&mut self) {
        for cursor in &mut self.cursors {
            if (cursor.col as usize) < self.lines[cursor.line as usize].len() {
                cursor.col += 1;
            }
            else if cursor.line as usize == self.lines.len() - 1 {
                return;
            }
            else {
                cursor.line += 1;
                cursor.col = 0;
            }
        }
    }

    pub fn right_line(&mut self) {
        for cursor in &mut self.cursors {
            while (cursor.col as usize) < self.lines[cursor.line as usize].len() {
                cursor.col += 1;
            }
        }
    }

    pub fn right_word(&mut self) {
        for cursor in &mut self.cursors {
            let line = &self.lines[cursor.line as usize];
            let chars: Vec<char> = line.chars().collect();
            let mut col = cursor.col as usize;

            if chars[col] == ' ' {
                col += 1;
            }

            while col < chars.len() && chars[col] != ' ' {
                col += 1;
            }

            cursor.col = col as u16;
        }
    }

    pub fn left_word(&mut self) {
        for cursor in &mut self.cursors {
            let line = &self.lines[cursor.line as usize];
            let chars: Vec<char> = line.chars().collect();
            let mut col = cursor.col as usize;
            
            if chars[col - 1] == ' ' && col > 0 {
                col -= 1;
            }

            while col > 0 && chars[col - 1] != ' ' {
                col -= 1;
            }

            cursor.col = col as u16;
        }
    }

    pub fn left_line(&mut self) {
        for cursor in &mut self.cursors {
            while (cursor.col as usize) > 0 {
                cursor.col -= 1;
            }
        }
    }

    pub fn left(&mut self) {
        for cursor in &mut self.cursors {
            if (cursor.col as usize) > 0 {
                cursor.col -= 1;
            }
            else if cursor.line == 0 {
                return;
            }
            else {
                cursor.line -= 1;
                cursor.col = self.lines[cursor.line as usize].len() as u16;
            }
        }
    }

    pub fn down_five(&mut self) {
        for _ in 0..5 {
            self.down();
        }
    }

    pub fn down(&mut self) {
        for cursor in &mut self.cursors {
            cursor.line += 1;
            if (cursor.line as usize) >= self.lines.len() {
                cursor.line = self.lines.len() as u16 - 1;
                cursor.col = self.lines[self.lines.len() - 1].len() as u16;
            }
            else {
                let current_line_len = self.lines[cursor.line as usize].len();
                if (cursor.col as usize) > current_line_len {
                    cursor.col = current_line_len as u16;
                }
            }
        }
    }

    pub fn up_five(&mut self) {
        for _ in 0..5 {
            self.up();
        }
    }

    pub fn up(&mut self) {
        for cursor in &mut self.cursors {
            if (cursor.line as usize) > 0 {
                cursor.line -= 1;

                if (self.lines[cursor.line as usize].len() as u16) < cursor.col {
                    cursor.col = self.lines[cursor.line as usize].len() as u16;
                }
            }
        }
    }
}