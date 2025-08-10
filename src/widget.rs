use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

pub fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    let mut input = String::new();

    loop {
        terminal.draw(|f| {
            draw_ui(f, &input);
        })?;

        // here your keymaps
        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    break;
                }
                (KeyCode::Char(c), _) => input.push(c),
                (KeyCode::Backspace, _) => {
                    input.pop();
                }
                _ => {}
            }
        }
    }
    Ok(())
}

#[allow(clippy::uninlined_format_args)]
pub fn draw_ui(f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, input: &str) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    // Header: input box
    let header = Paragraph::new(input.to_string())
        .block(Block::default().title("Input").borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));

    let main = Block::default().borders(Borders::ALL);

    // Footer
    let span = Spans::from(Span::styled(
        "ctrl + q to exit",
        Style::default().fg(Color::Yellow),
    ));
    let footer = Paragraph::new(span)
        .block(Block::default().borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left);

    f.render_widget(header, chunks[0]);
    f.render_widget(main, chunks[1]);
    f.render_widget(footer, chunks[2]);
}
