// handle ctrl + moves in this file
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use super::{
    editor::Editor,
    commands::Command
};
use crate::files::save::save_file;

/**
 * Handle any CTRL + key events
 * 
 * @param text: &mut String - Mutable reference to the current text in the editor
 * @param code: KeyCode - The key code of the key event
 * @param modifier: KeyModifiers - The modifier of the key event (should be CTRL, maybe more?)
 * We should probably pass in current state of the editor too, if we want to move the cursor
 */
pub fn handle_ctrl(editor: &mut Editor, code: KeyCode, modifier: KeyModifiers) -> () {
    match code {
        KeyCode::Char('c') => {
            editor.notif_text = String::from("Copy from clipboard");
        }
        KeyCode::Char('v') => {
            editor.notif_text = String::from("Paste from clipboard");
        }
        KeyCode::Char('s') => {
            save_file(&editor);
            editor.notif_text = String::from("File saved");
        }
        KeyCode::Char(':') => {
            editor.notif_text = String::from("Command mode");
        }
        KeyCode::Char('p') => {
            editor.notif_text = String::from("Open File");
        }
        KeyCode::Char('g') => {
            editor.notif_text = String::from("Goto line: ");
            editor.command_mode = true;
            editor.command = Command::GOTO_LINE;
        }
        KeyCode::Char('j') => { // for some reason, CTRL + Enter maps to CTRL + j?
            editor.notif_text = String::from("Move cursor with WASD");
            editor.command_mode = true;
            editor.command = Command::MOVE_CURSOR;
        }
        KeyCode::Char('l') => {
            editor.notif_text = String::from("Select line");
        }
        KeyCode::Char('d') => {
            editor.notif_text = String::from("Select word / duplicates");
        }
        KeyCode::Char('f') => {
            editor.notif_text = String::from("Find occurence");
        }
        KeyCode::Right => {
            for cursor in &mut editor.cursors {
                while (cursor.col as usize) < editor.lines[cursor.line as usize].len() {
                    cursor.col += 1;
                }
            }
        }
        KeyCode::Left => {
            while (editor.cursor.1 as usize) > 0 {
                editor.cursor.1 -= 1;
            }
        }
        _ => {
            editor.notif_text = String::from("Invalid command");
        }
    }
}

pub fn handle_command(editor: &mut Editor, code: KeyCode, modifier: KeyModifiers) -> () {
    if editor.command == Command::MOVE_CURSOR {
        match code {
            KeyCode::Char('w') => {
                editor.up();
            }
            KeyCode::Char('s') => {
                editor.down();
            }
            KeyCode::Char('a') => {
                editor.left();
            }
            KeyCode::Char('d') => {
                editor.right();
            }
            _ => {}
        }

        return;
    }

    match code {
        KeyCode::Char(c) => {
            editor.notif_text.push(c);
        }
        KeyCode::Backspace => {
            editor.notif_text.pop();
        }
        KeyCode::Esc => {
            editor.command_mode = false;
            editor.notif_text = String::new();
        }
        KeyCode::Enter => {
            match editor.command {
                Command::GOTO_LINE => {
                    editor.command_mode = false;
                    
                    // only keep one cursor
                    if editor.cursors.len() > 1 {
                        editor.cursors = vec![editor.cursors[0].clone()];
                    }


                    //split string into parts
                    let string_parts: Vec<&str> = editor.notif_text.split(':').collect();
                    if string_parts.len() < 2 {
                        editor.notif_text = String::from("Invalid line number!");
                        return;
                    }

                    //parse line number
                    let mut line: u16 = string_parts[1].trim().parse().unwrap();
                    if line > editor.lines.len() as u16 {
                        line = editor.lines.len() as u16;
                    }
                    editor.cursors[0].line = line - 1;

                    //parse column numbner
                    if string_parts.len() > 2 {
                        let mut col: u16 = string_parts[2].trim().parse().unwrap();
                        
                        if col > editor.lines[line as usize - 1].len() as u16 {
                            col = editor.lines[line as usize - 1].len() as u16;
                        } 
                        editor.cursors[0].col = col;
                    }
                }
                Command::MOVE_CURSOR => {
                    editor.command_mode = false;
                }
                _ => {}
            }
        }
        _ => {}
    }
}