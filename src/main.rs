mod files;
mod editor;

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Text, Line},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    env, io::{self, stdout}
};

use files::open::open_file;

use editor::{
    editor::Editor,
    input::{
        handle_ctrl, handle_command
    },
};

fn main() -> io::Result<()> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //define editor state
    let mut editor = Editor::new();

    //get filename and populate the editor
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        editor.filename = args[1].clone();
        open_file(&mut editor);
    }

    let res = run_app(&mut terminal, &mut editor);
    // let res = Ok(());

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;


    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, editor: &mut Editor) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let size = frame.area();
    
            // outer layout to add the header and the editor
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)  // Vertical split
                .constraints([Constraint::Length(3), Constraint::Min(20)]) 
                .split(size);


            // split the header into two blocks
            let header_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(outer_layout[0]);
            
            let header_block = Block::default()
                .borders(Borders::ALL);

            let title_left_as_text = Text::from(String::from("Peter's Editor: ") + &editor.filename.clone());
            let header_left = Paragraph::new(title_left_as_text)
                .block(header_block.clone())
                .style(Style::default().fg(Color::Cyan))
                .alignment(ratatui::layout::Alignment::Center);

            let title_right_as_text = Text::from(editor.notif_text.clone());
            let header_right = Paragraph::new(title_right_as_text)
                .block(header_block.clone())
                .alignment(ratatui::layout::Alignment::Center);

            frame.render_widget(header_block, outer_layout[0]);
            frame.render_widget(header_left, header_layout[0]);
            frame.render_widget(header_right, header_layout[1]);


            let editor_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(6), Constraint::Min(0)])
                .split(outer_layout[1]);

            // add cursor to editor text
            let mut lines_with_cursor = Vec::new();
            let mut line_numbers = Vec::new();
            let lines = &editor.lines;
            
            // get cursor position
            let cursor_line = editor.cursors[0].line as usize;
            let col = editor.cursors[0].col as usize;
            for (index, line) in lines.iter().enumerate() {
                line_numbers.push(Line::styled(
                    format!("{:4}  ", index + 1),
                    Style::default().fg(Color::Cyan),
                ));

                if index == cursor_line {
                    // Insert cursor symbol (`^`) at the correct column
                    let mut line_with_cursor = line.to_string();
                    if col < line_with_cursor.len() {
                        line_with_cursor.insert(col, '^'); // cursor position
                    } else {
                        line_with_cursor.push('^'); // cursor at the end of the line
                    }
                    lines_with_cursor.push(Line::raw(line_with_cursor));
                } else {
                    lines_with_cursor.push(Line::raw(line.clone()));
                }
            }

            // create text and lines for the editor
            let editor_text = Text::from(lines_with_cursor);
            let editor_paragraph = Paragraph::new(editor_text)
                .block(Block::default())
                .alignment(ratatui::layout::Alignment::Left);

            let lines_text = Text::from(line_numbers);
            let lines_paragraph = Paragraph::new(lines_text)
                .block(Block::default())
                .alignment(ratatui::layout::Alignment::Right);

            // Render the blocks in the nested layout
            frame.render_widget(lines_paragraph, editor_layout[0]);
            frame.render_widget(editor_paragraph, editor_layout[1]);
        })?;

        if let event::Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
            if editor.command_mode {
                handle_command(editor, code, modifiers);
                continue;
            }
            match (code, modifiers) {
                (_, KeyModifiers::CONTROL) | (_, KeyModifiers::META) => {
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        //handle_ctrl_shift(editor, code, modifiers);
                    }
                    handle_ctrl(editor, code, modifiers);
                }
                (KeyCode::Tab, _) => {
                    editor.insert('\t');
                }
                (KeyCode::Backspace, _) => {
                    editor.backspace();
                }
                (KeyCode::Enter, _) => {
                    editor.new_line();
                }
                (KeyCode::Esc, _) => {
                    break;
                }
                (KeyCode::Char(' '), _) => {
                    editor.insert(' ');
                }
                (KeyCode::Right, _) => {
                    editor.right();
                }
                (KeyCode::Left, _) => {
                    editor.left();
                }
                (KeyCode::Up, _) => {
                    editor.up();
                }
                (KeyCode::Down, _) => {
                    editor.down()
                }
                _ => {
                    editor.insert(code.to_string().chars().next().unwrap());
                }
            }
        }
    }
    Ok(())
}