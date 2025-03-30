use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)])
        .split(size);

    let block = Block::default()
        .title(app.title.as_str())
        .borders(Borders::ALL);

    let paragraph = Paragraph::new("Press 'q' to quit.").block(block);

    frame.render_widget(paragraph, layout[0]);
}
