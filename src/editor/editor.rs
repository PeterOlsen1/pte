use super::{
    cursor::Cursor,
    commands::Command
};

pub struct Editor {
    pub text: String,
    pub lines: Vec<String>,
    pub cursors: Vec<Cursor>,
    pub filename: String,
    pub notif_text: String,
    pub command_mode: bool,
    pub command: Command,
}

impl Editor {
    pub fn new() -> Self {
        let mut temp = Self {
            text: String::new(),
            lines: Vec::new(),
            cursors: Vec::new(),
            filename: String::new(),
            notif_text: String::from("Editor mode"),
            command_mode: false,
            command: Command::new(),
        };

        temp.cursors.push(Cursor::new());
        temp
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
        for cursor in &mut self.cursors {
            let line = cursor.line as usize;
            let col = cursor.col as usize;
    
            if col > 0 {
                let line = &mut self.lines[cursor.line as usize];
                line.remove(col as usize - 1);
                cursor.col -= 1;
            } 
            else if line == 0 {
                return;
            } 
            else if line > 0 {
                let prev_len = self.lines[cursor.line as usize - 1].len();
                let line = self.lines[cursor.line as usize].clone();
    
                self.lines[cursor.line as usize - 1].push_str(&line);
                self.lines.remove(cursor.line as usize);
                cursor.line -= 1;
                cursor.col = prev_len as u16;
            }
        }
    }

    pub fn insert(&mut self, c: char) {
        for cursor in &mut self.cursors {
            let cursor_x = cursor.col as usize;
            let line = &mut self.lines[cursor.line as usize];
            line.insert(cursor_x, c);
            cursor.col += 1;
        }
    }

    pub fn new_line(&mut self) {
        for cursor in &mut self.cursors {
            let cursor_x = cursor.col as usize;
            let line = &mut self.lines[cursor.line as usize];
            let new_line = line.split_off(cursor_x);
            self.lines.insert(cursor.line as usize + 1, new_line);
            cursor.line += 1;
            cursor.col = 0;
        }
    }

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

    pub fn right_word(&mut self) {
        for cursor in &mut self.cursors {
            while (cursor.col as usize) < self.lines[cursor.line as usize].len() {
                cursor.col += 1;
            }
        }
    }

    pub fn left_word(&mut self) {
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