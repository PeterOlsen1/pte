use std::{
    fs::OpenOptions,
    io::Write
};

/// use this function to debug to a file when
/// we can't debug to the editor
pub fn dbg(line: &str) -> () {
    let mut file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open("debug_file.txt") {
            Ok(file) => file,
            Err(_) => return,
        };

    writeln!(file, "{}", line);
}