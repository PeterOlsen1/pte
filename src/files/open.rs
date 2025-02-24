use crate::Editor;
use std::{
    fs::OpenOptions, 
    io::Read,
    path::Path
};

pub fn open_file(editor: &mut Editor) {
    let file_path = Path::new(&editor.filename);
    let file_exists = file_path.exists();

    let file = OpenOptions::new().read(true).write(true).create(true).open(&editor.filename);
    match file {
        Ok(mut file) => {

            //read contents of the file
            let mut contents = String::new();
            let read = file.read_to_string(&mut contents);
            match read {
                Ok(_) => {
                    editor.text = contents;
                    editor.lines = editor.text.lines().map(|s| s.to_string()).collect();
                    editor.notif_text = String::from("Edit mode");
                    editor.command_mode = false;
                    editor.history.clear();

                    editor.cursors = vec![editor.cursors[0].clone()];
                    editor.cursors[0].line = 0;
                    editor.cursors[0].col = 0;
                }
                Err(_) => {
                    editor.text = String::from("Error reading file");
                    editor.notif_text = String::from("Error reading file");
                }
            }
        },
        Err(_) => {
            editor.notif_text = String::from("Failed to open file!");
            editor.lines = Vec::new();
            editor.lines.push(String::from("Failed to open file!"));
        }
    }

    if !file_exists {
        editor.notif_text = String::from("New file created");
    }
    // else {
}