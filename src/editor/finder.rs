use super::editor::Editor;

pub struct Finder {
    pub query: String,
    pub search_index: usize,
    pub search_results: Vec<(u16, u16)>,
}

impl Finder {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            search_index: 0,
            search_results: Vec::new(),
        }
    }

    pub fn find(&mut self, lines: Vec<String>, current_line: u16) {
        self.search_results.clear();

        for (i, line) in lines.iter().enumerate() {
            let mut line_clone = line.clone();
            line_clone.make_ascii_lowercase();
            let mut query_clone = self.query.clone();
            query_clone.make_ascii_lowercase();
            let mut chopped_len = 0;

            while line_clone.contains(&query_clone) {
                let index = line_clone.find(&query_clone).unwrap();
                self.search_results.push((i as u16, index as u16 + chopped_len + query_clone.len() as u16));
                chopped_len += (index + query_clone.len()) as u16;
                line_clone = line_clone.split_off(index + query_clone.len());
            }
        }
    }

    pub fn next(&mut self) {
        if self.search_results.len() == 0 {
            return;
        }

        self.search_index += 1;
        if self.search_index >= self.search_results.len() {
            self.search_index = 0;
        }
    }

    pub fn prev(&mut self) {
        if self.search_results.len() == 0 {
            return;
        }

        if self.search_index == 0 {
            self.search_index = self.search_results.len() - 1;
        } else {
            self.search_index -= 1;
        }
    }
}