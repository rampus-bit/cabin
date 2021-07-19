extern crate termion;

mod ui;
mod util;

use crate::util::{Event, Events};

use std::io;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::widgets::{Block, Borders, Paragraph};
use tui::{backend::TermionBackend, Terminal};
use tui::layout::{Alignment, Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ]
                    .as_ref()
                )
                .split(f.size());

            let text = vec![
                Spans::from("This is a line "),
                Spans::from(Span::styled("This is a line   ", Style::default().fg(Color::Red))),
                Spans::from(Span::styled("This is a line", Style::default().bg(Color::Blue))),
                Spans::from(Span::styled(
                    "This is a longer line",
                    Style::default().add_modifier(Modifier::CROSSED_OUT),
                )),
                Spans::from(Span::styled(
                    "This is a line",
                    Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC),
                )),
            ];

            let create_block = |title| {
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::White).fg(Color::Black))
                    .title(Span::styled(title, Style::default().add_modifier(Modifier::BOLD)))
            };

            let paragraph = Paragraph::new(text.clone())
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .block(create_block("Left, no wrap"))
                .alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[0]);

            let block = Block::default()
                .title("Block 2")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
        }
    }
}
