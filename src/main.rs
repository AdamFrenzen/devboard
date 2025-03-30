mod app;
mod input;
mod ui;

use crate::app::App;
use crate::input::watch_keys;
use crate::ui::draw;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};

fn main() -> io::Result<()> {
    // setup terminal UI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run the app loop
    let res = run_app(&mut terminal);

    // cleanup terminal on exit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| draw(f, &app))?;

        let size = terminal.size()?;
        let button_area_width = size.width.saturating_sub(4);

        if watch_keys(&mut app, button_area_width)? {
            break;
        }
    }

    Ok(())
}
