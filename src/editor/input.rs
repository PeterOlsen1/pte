// handle ctrl + moves in this file
use crossterm::event::{KeyCode, KeyModifiers};
use super::{
    editor::Editor,
    commands::Command
};
use crate::files::save::save_file;

/**
 * Handle any CTRL + key events
 * 
 * @param editor: &mut Editor - Mutable reference to the current editor
 * @param code: KeyCode - The key code of the key event
 * @param modifier: KeyModifiers - The modifier of the key event (should be CTRL, maybe more?)
 * We should probably pass in current state of the editor too, if we want to move the cursor
 */
pub fn handle_ctrl(editor: &mut Editor, code: KeyCode, modifier: KeyModifiers) -> () {

    match code {
        KeyCode::Char('z') => {
            editor.notif_text = String::from("Undo");
            editor.undo();
        }
        KeyCode::Char('c') => {
            editor.notif_text = String::from("Copy from clipboard");
        }
        KeyCode::Char('v') => {
            editor.notif_text = String::from("Paste from clipboard");
            editor.insert_string("Hello, world!".to_string());
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
            editor.command = Command::GotoLine;
        }
        KeyCode::Char('j') => { // for some reason, CTRL + Enter maps to CTRL + j?
            editor.notif_text = String::from("Move cursor with WASD");
            editor.command_mode = true;
            editor.command = Command::MoveCursor;
        }
        KeyCode::Char('l') => {
            editor.notif_text = String::from("Select line");
        }
        KeyCode::Char('d') => {
            editor.notif_text = String::from("Select word / duplicates");
        }
        KeyCode::Char('f') => {
            editor.notif_text = String::from("Find substring: ");
            editor.command_mode = true;
            editor.command = Command::Find;
        }
        KeyCode::Char('h') => { // CTRL + Backspace maps to CTRL + h
            editor.notif_text = String::from("Delete line");
            editor.backspace_line();
        }
        KeyCode::Right => {
            editor.right_line();
        }
        KeyCode::Left => {
            editor.left_line();
        }
        _ => {
            editor.notif_text = String::from("Invalid command");
        }
    }
}

/**
 * Handle any CTRL + SHIFT + key events
 * 
 * @param editor: &mut Editor - Mutable reference to the current editor
 * @param code: KeyCode - The key code of the key event
 * @param modifier: KeyModifiers - The modifier of the key event (should be CTRL, maybe more?)
 */
pub fn handle_ctrl_shift(editor: &mut Editor, code: KeyCode, modifier: KeyModifiers) -> () {
    match code {
        KeyCode::Right => {
            //
        }
        KeyCode::Left => {

        }
        KeyCode::Up => {

        }
        KeyCode::Down => {

        }
        _ => {}
    }
}

pub fn handle_command(editor: &mut Editor, code: KeyCode, modifier: KeyModifiers) -> () {
    if editor.command == Command::MoveCursor {
        match code {
            KeyCode::Char('w') | KeyCode::Char('i') | KeyCode::Up => {
                editor.up();
            }
            KeyCode::Char('s') | KeyCode::Char('k') | KeyCode::Down => {
                editor.down();
            }
            KeyCode::Char('a') | KeyCode::Char('j') | KeyCode::Left => {
                if modifier == KeyModifiers::CONTROL {
                    editor.left_line();
                }
                else {
                    editor.left();
                }
            }
            KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
                if modifier == KeyModifiers::CONTROL {
                    editor.right_line();
                } 
                else {
                    editor.right();
                }
            }
            KeyCode::Enter | KeyCode::Esc => {
                editor.command_mode = false;
                editor.notif_text = String::from("Editor mode");
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
                Command::GotoLine => {
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
                Command::Find => {
                    editor.command_mode = false;

                    // only keep one cursor
                    if editor.cursors.len() > 1 {
                        editor.cursors = vec![editor.cursors[0].clone()];
                    }

                    let query: String = editor.notif_text.split_off("Find substring: ".len());
                    if query.len() == 0 {
                        editor.notif_text = String::from("Invalid substring!");
                        return;
                    }

                    for i in 0..editor.lines.len() {
                        if editor.lines[i].contains(&query) {
                            editor.cursors[0].line = i as u16;
                            editor.cursors[0].col = editor.lines[i].find(&query).unwrap() as u16;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}