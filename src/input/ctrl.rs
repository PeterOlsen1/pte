// handle ctrl + moves in this file
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::Editor;
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
    match (code, modifier) {
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            editor.notification = String::from("Copy from clipboard");
        }
        (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
            editor.notification = String::from("Paste from clipboard");
        }
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
            save_file(&editor);
            editor.notification = String::from("File saved");
        }
        (KeyCode::Char(':'), KeyModifiers::CONTROL) => {
            editor.notification = String::from("Command mode");
        }
        _ => {}
    }
}