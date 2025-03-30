mod app;
mod ui;

use crate::app::App;
use crate::app::Mode;
use crate::ui::draw;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn handle_insert_keys(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Char(c) => app.input.push(c),
        KeyCode::Backspace => {
            app.input.pop();
        }
        _ => {}
    }
}
fn handle_normal_keys(app: &mut App, key: KeyCode, view_width: u16) -> bool {
    match key {
        KeyCode::Char('q') => return false,
        KeyCode::Char('h') => app.previous(view_width),
        KeyCode::Char('l') => app.next(view_width),
        KeyCode::Char('i') => app.mode = Mode::Insert,
        KeyCode::Enter => {
            println!("Running: {}", app.selected_command());
        }
        _ => {}
    }
    return true;
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();
    let size = terminal.size()?; // <- gives you terminal Rect
    let button_area_width = size.width.saturating_sub(4); // for borders/margins

    loop {
        terminal.draw(|f| draw(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Insert => handle_insert_keys(&mut app, key.code),
                    Mode::Normal => {
                        if !handle_normal_keys(&mut app, key.code, button_area_width) {
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
