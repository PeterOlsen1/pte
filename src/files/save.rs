use crate::Editor;
use std::io::Write;

pub fn save_file(editor: &Editor) {
    let file = std::fs::File::create(&editor.filename);

    let content = editor.lines.join("\n");
    match file {
        Ok(mut file) => {
            let write = file.write_all(content.as_bytes());
            match write {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}