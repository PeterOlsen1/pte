pub struct Editor {
    pub text: String,
    pub lines: Vec<String>,
    pub cursor: (u16, u16),
    pub filename: String,
    pub notification: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            lines: Vec::new(),
            cursor: (0, 0),
            filename: String::new(),
            notification: String::new(),
        }
    }

    /**
     * Get a mutable reference to the line we want to work on
     */
    pub fn get_line(&mut self) -> &mut String {
        &mut self.lines[self.cursor.0 as usize]
    }

    pub fn backspace(&mut self) {
        let cursor_y = self.cursor.0 as usize;
        let cursor_x = self.cursor.1 as usize;

        if cursor_x > 0 {
            let line = self.get_line();
            line.remove(cursor_x as usize - 1);
            self.cursor.1 -= 1;
        } 
        else if cursor_y == 0 {
            return;
        } 
        else if self.cursor.0 > 0 {
            let prev_len = self.lines[self.cursor.0 as usize - 1].len();
            let line = self.get_line().clone();

            self.lines[self.cursor.0 as usize - 1].push_str(&line);
            self.lines.remove(self.cursor.0 as usize);
            self.cursor.0 -= 1;
            self.cursor.1 = prev_len as u16;
        }
    }

    pub fn insert(&mut self, c: char) {
        let cursor_x = self.cursor.1 as usize;
        let line = self.get_line();
        line.insert(cursor_x, c);
        self.cursor.1 += 1;
    }

    pub fn new_line(&mut self) {
        let cursor_x = self.cursor.1 as usize;
        let line = self.get_line();
        let new_line = line.split_off(cursor_x);
        self.lines.insert(self.cursor.0 as usize + 1, new_line);
        self.cursor.0 += 1;
        self.cursor.1 = 0;
    }

    pub fn right(&mut self) {
        if (self.cursor.1 as usize) < self.lines[self.cursor.0 as usize].len() {
            self.cursor.1 += 1;
        }
        else if self.cursor.0 as usize == self.lines.len() - 1 {
            return;
        }
        else {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
    }

    pub fn left(&mut self) {
        if (self.cursor.1 as usize) > 0 {
            self.cursor.1 -= 1;
        }
        else if self.cursor.0 == 0 {
            return;
        }
        else {
            self.cursor.0 -= 1;
            self.cursor.1 = self.lines[self.cursor.0 as usize].len() as u16;
        }
    }

    pub fn down(&mut self) {
        self.cursor.0 += 1;
        if (self.cursor.0 as usize) >= self.lines.len() {
            self.cursor.0 = self.lines.len() as u16 - 1;
            self.cursor.1 = self.lines[self.lines.len() - 1].len() as u16;
        }
        else {
            let current_line_len = self.lines[self.cursor.0 as usize].len();
            if (self.cursor.1 as usize) > current_line_len {
                self.cursor.1 = current_line_len as u16;
            }
        }
    }

    pub fn up(&mut self) {
        if (self.cursor.0 as usize) > 0 {
            self.cursor.0 -= 1;

            if (self.lines[self.cursor.0 as usize].len() as u16) < self.cursor.1 {
                self.cursor.1 = self.lines[self.cursor.0 as usize].len() as u16;
            }
        }
        else {
            self.cursor.1 = 0;
        }
    }
}