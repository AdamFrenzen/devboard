use crate::app::App;
use crate::app::Mode;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const RAINBOW: [Color; 6] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
];

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // commands
            Constraint::Length(3), // input
            Constraint::Min(5),    // output
            Constraint::Length(1), // output
        ])
        .split(size);

    // 1️⃣ Commands box (top)
    let title = Line::from(vec![Span::styled(
        " dashboard ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]);

    let button_line = Line::from(
        app.commands
            .iter()
            .enumerate()
            .flat_map(|(i, cmd)| {
                let style = if i == app.selected_index && app.mode == Mode::Normal {
                    Style::default()
                        .fg(Color::Gray)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(RAINBOW[i % RAINBOW.len()])
                };

                vec![
                    Span::raw(" "),
                    Span::styled(format!("[{}]", cmd), style),
                    Span::raw(" "),
                ]
            })
            .collect::<Vec<Span>>(),
    );

    let commands = Paragraph::new(button_line)
        .block(Block::default().borders(Borders::ALL).title(title))
        .scroll((0, app.scroll_offset));
    f.render_widget(commands, layout[0]);

    // 2️⃣ Input box (middle)
    // let input = Paragraph::new("> your input command here")
    //     .block(Block::default().borders(Borders::ALL).title(" Input "));
    // f.render_widget(input, layout[1]);

    let input_text = match app.mode {
        Mode::Insert => format!("> {}", app.input),
        Mode::Normal => format!("> {}", app.input), // maybe fade later
    };

    let input = Paragraph::new(Line::from(Span::raw(input_text)))
        .block(Block::default().borders(Borders::ALL).title(" Input "));
    f.render_widget(input, layout[1]);

    // 3️⃣ Output box (bottom)
    let output = Paragraph::new("Build succeeded.\nRunning...\nHello, world!")
        .block(Block::default().borders(Borders::ALL).title(" Output "));
    f.render_widget(output, layout[2]);

    let help_line = if app.mode == Mode::Insert {
        " [esc] normal mode "
    } else {
        " [i] insert [s] save input [h] help [q] quit"
    };
    let footer = Paragraph::new(Line::from(vec![Span::styled(
        help_line,
        Style::default().fg(Color::DarkGray),
    )]));

    f.render_widget(footer, layout[3]);

    if app.mode == Mode::Insert {
        let x = (app.input.len() + 2) as u16;
        let y = layout[1].y + 1;
        f.set_cursor(x, y);
    }
}
