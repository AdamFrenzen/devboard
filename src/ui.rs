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

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // command_box
            Constraint::Length(3), // input_box
            Constraint::Min(5),    // output_box
            Constraint::Length(1), // help line
        ])
        .split(size);

    frame.render_widget(command_box(app), layout[0]);
    frame.render_widget(input_box(app), layout[1]);
    frame.render_widget(output_box(app), layout[2]);
    frame.render_widget(help_line(app), layout[3]);

    if app.mode == Mode::Insert {
        let x = (app.input.len() + 3) as u16;
        let y = layout[1].y + 1;
        frame.set_cursor(x, y);
    }
}

fn command_box(app: &App) -> Paragraph<'_> {
    let title = Line::from(vec![Span::styled(
        &app.title,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]);

    let button_line = match app.commands.is_empty() {
        true => Line::styled(
            " [i]nsert command -> [esc] normal mode -> [s]ave",
            Style::default().fg(Color::DarkGray),
        ),
        false => Line::from(
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
        ),
    };

    Paragraph::new(button_line)
        .block(Block::default().borders(Borders::ALL).title(title))
        .scroll((0, app.scroll_offset))
}

fn input_box(app: &App) -> Paragraph<'_> {
    let input_text = format!("> {}", app.input);
    Paragraph::new(Line::from(Span::raw(input_text)))
        .block(Block::default().borders(Borders::ALL).title(" Input "))
}

fn output_box(app: &App) -> Paragraph<'_> {
    Paragraph::new(Span::raw(&app.output))
        .block(Block::default().borders(Borders::ALL).title(" Output "))
}

fn help_line(app: &App) -> Paragraph<'_> {
    let help_text = match app.mode {
        Mode::Insert => " [esc] normal mode ",
        Mode::Normal => " [i] insert [s] save input [h] help [q] quit",
    };

    Paragraph::new(Line::from(vec![Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray),
    )]))
}
