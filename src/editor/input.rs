// handle ctrl + moves in this file
use crossterm::event::{KeyCode, KeyModifiers};
use super::{
    editor::Editor,
    commands::Command,
    finder::Finder
};

use std::{env, fs};

use crate::files::{
    open::open_file, save::save_file
};

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
            //reset finder object
            editor.finder = Finder::new();
            editor.notif_text = String::from("Find substring: ");
            editor.command_mode = true;
            editor.command = Command::Find;
        }
        KeyCode::Char('h') => { // CTRL + Backspace maps to CTRL + h
            editor.notif_text = String::from("Delete line");
            editor.backspace_line();
        }
        KeyCode::Char('o') => {
            editor.notif_text = String::from("Open file: ");
            editor.command_mode = true;
            editor.command = Command::OpenFile;
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
            for cursor in &mut editor.cursors {
                let mut new_col = cursor.col + 1;
                if new_col > editor.lines[cursor.line as usize].len() as u16 {
                    new_col = editor.lines[cursor.line as usize].len() as u16;
                }
                cursor.expand_selection(cursor.line, new_col);
            }
        }
        KeyCode::Left => {
            for cursor in &mut editor.cursors {
                if cursor.col == 0 {
                    continue;
                }
                cursor.expand_selection(cursor.line, cursor.col - 1);
            }
        }
        KeyCode::Up => {
            for cursor in &mut editor.cursors {
                if cursor.line == 0 {
                    continue;
                }
                cursor.expand_selection(cursor.line - 1, cursor.col);
            }
        }
        KeyCode::Down => {
            for cursor in &mut editor.cursors {
                let mut new_line = cursor.line + 1;
                if new_line > editor.lines.len() as u16 {
                    new_line = editor.lines.len() as u16;
                }
                cursor.expand_selection(new_line, cursor.col);
            }
        }
        _ => {}
    }
}

pub fn handle_command(editor: &mut Editor, code: KeyCode, modifier: KeyModifiers) -> () {
    
    //match for commands that don't need character input
    match editor.command {
        Command::MoveCursor => {
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
        Command::FindSelection => {
            match code {
                KeyCode::Right | KeyCode::Down => {
                    editor.finder.next();
                    editor.cursors[0].line = editor.finder.search_results[editor.finder.search_index].0;
                    editor.cursors[0].col = editor.finder.search_results[editor.finder.search_index].1;
                    editor.notif_text = format!("{} of {}", editor.finder.search_index + 1, editor.finder.search_results.len());
                }
                KeyCode::Left | KeyCode::Up => {
                    editor.finder.prev();
                    editor.cursors[0].line = editor.finder.search_results[editor.finder.search_index].0;
                    editor.cursors[0].col = editor.finder.search_results[editor.finder.search_index].1;
                    editor.notif_text = format!("{} of {}", editor.finder.search_index + 1, editor.finder.search_results.len());
                }
                KeyCode::Enter => {
                    editor.cursors[0].line = editor.finder.search_results[editor.finder.search_index].0;
                    editor.cursors[0].col = editor.finder.search_results[editor.finder.search_index].1;
                    editor.command_mode = false;
                    editor.notif_text = String::from("Editor mode");
                }
                KeyCode::Esc => {
                    editor.command_mode = false;
                    editor.notif_text = String::from("Editor mode");
                }
                _ => {
                    editor.notif_text = String::from("Invalid command. Use arrow keys to navigate search results");
                }
            }
            return;
        }
        Command::OpenFile => {
            if code == KeyCode::Tab {
                let dir =  match env::current_dir() {
                    Ok(dir) => dir,
                    Err(_) => {
                        editor.notif_text = String::from("Error getting current directory");
                        editor.command_mode = false;
                        return;
                    }
                };
    
                // editor.notif_text = format!("Current directory: {}", dir.display());
                fs::read_dir(dir).unwrap().for_each(|entry| {
                    match entry {
                        Ok(entry) => {
                            editor.dbg(entry.file_name().to_str().unwrap().to_string());
                        }
                        Err(_) => {}
                    }
                });
            }
        }
        _ => {}
    }

    match code {
        KeyCode::Char(c) => {
            if editor.command == Command::SavePrompt {
                match c {
                    'y' => {
                        save_file(editor);
                        editor.notif_text = String::from("File saved");
                        editor.filename = editor.file_to_open.clone();
                        open_file(editor);
                    }
                    'n' => {
                        editor.filename = editor.file_to_open.clone();
                        open_file(editor);
                    }
                    _ => {}
                }
                return;
            }
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
                    // only keep one cursor
                    if editor.cursors.len() > 1 {
                        editor.cursors = vec![editor.cursors[0].clone()];
                    }

                    let query: String = editor.notif_text.split_off("Find substring: ".len());
                    if query.len() == 0 {
                        editor.notif_text = String::from("Invalid substring!");
                        return;
                    }

                    editor.finder.query = query;
                    editor.finder.find(editor.lines.clone(), editor.cursors[0].line);

                    if editor.finder.search_results.is_empty() {
                        editor.notif_text = String::from("No results found");
                        editor.command_mode = false;
                        return;
                    }

                    editor.command = Command::FindSelection;
                    editor.cursors[0].line = editor.finder.search_results[editor.finder.search_index].0;
                    editor.cursors[0].col = editor.finder.search_results[editor.finder.search_index].1;
                    editor.notif_text = format!("{} of {}", editor.finder.search_index + 1, editor.finder.search_results.len());

                    // for i in 0..editor.lines.len() {
                    //     if editor.lines[i].contains(&query) {
                    //         editor.cursors[0].line = i as u16;
                    //         editor.cursors[0].col = editor.lines[i].find(&query).unwrap() as u16;
                    //         break;
                    //     }
                    // }
                }
                Command::OpenFile => {
                    editor.command_mode = false;
                    
                    // only keep one cursor
                    if editor.cursors.len() > 1 {
                        editor.cursors = vec![editor.cursors[0].clone()];
                        editor.cursors[0].col = 0;
                        editor.cursors[0].line = 0;
                    }

                    let query: String = editor.notif_text.split_off("Open fille:".len());
                    if query.len() == 0 {
                        editor.notif_text = String::from("Invalid filename!");
                        return;
                    }

                    if !editor.changes_saved {
                        editor.notif_text = String::from("Save changes before opening a new file? (y/n)");
                        editor.command_mode = true;
                        editor.command = Command::SavePrompt;
                        editor.file_to_open = query;
                        return;
                    }

                    save_file(editor);
                    editor.filename = query;
                    open_file(editor);
                }
                _ => {}
            }
        }
        _ => {}
    }
}