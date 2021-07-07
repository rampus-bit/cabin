extern crate termion;

use std::io;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::widgets::{Widget, Block, Borders};
use tui::{backend::TermionBackend, Terminal};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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
                    ].as_ref()
                )
                .split(f.size());
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default()
                .title("Block 2")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;
    }
    Ok(())
}
