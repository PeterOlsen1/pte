
///
/// ### Gets the length of a line as u16
/// 
/// Must be used with a valid Editor and Cursor object
/// To use with an integer, use the get_line_len_int! macro
/// 
/// expands to:
/// ```rust
/// editor.lines[cursor.line as usize].len() as u16
/// ```
#[macro_export]
macro_rules! get_line_len {
    ($editor:expr, $cursor:expr) => {
        $editor.lines[$cursor.line as usize].len() as u16
    };
}

///
/// ### Gets the length of a line as u16
/// 
/// Must be used with a valid Editor and int object
/// 
/// expands to:
/// ```rust
/// editor.lines[line as usize].len() as u16
/// ```
#[macro_export]
macro_rules! get_line_len_int {
    ($editor:expr, $line:expr) => {
        $editor.lines[$line as usize].len() as u16
    };
}

///
/// ### Gets the length of the lines vector as u16
/// 
#[macro_export]
macro_rules! get_lines_len {
    ($editor:expr) => {
        $editor.lines.len() as u16
    };
}