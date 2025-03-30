use crate::app::App;
use crate::app::Mode;
use crossterm::event::{self, Event, KeyCode};

pub fn watch_keys(app: &mut App, view_width: u16) -> std::io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match app.mode {
                // --- NORMAL MODE ---
                Mode::Normal => match key.code {
                    // [q] quit - exit the program
                    KeyCode::Char('q') => return Ok(true),
                    // [i] insert - enter insert mode to write commands
                    KeyCode::Char('i') => app.mode = Mode::Insert,
                    // [h] left - select command to the left
                    KeyCode::Char('h') => app.previous_command(view_width),
                    // [l] right - select command to the right
                    KeyCode::Char('l') => app.next_command(view_width),
                    // [ret] run - run the selected command
                    KeyCode::Enter => app.run_selected_command(),
                    KeyCode::Char('s') => app.save_command(),
                    // [d] or [x] delete - delete selected command
                    KeyCode::Char('d') => app.delete_selected_command(),
                    KeyCode::Char('x') => app.delete_selected_command(),
                    _ => {}
                },
                // --- INSERT MODE ---
                Mode::Insert => match key.code {
                    // [esc] normal - enter normal mode and manage commands
                    KeyCode::Esc => app.mode = Mode::Normal,
                    // <key> - write key to input
                    KeyCode::Char(c) => app.input.push(c),
                    // <bspc> - delete char from input
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    // [ret] run - run the input command without saving
                    KeyCode::Enter => app.run_input(),
                    _ => {}
                },
            }
        }
    }

    Ok(false)
}
