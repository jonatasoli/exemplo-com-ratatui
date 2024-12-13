use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{style::Stylize, widgets::Paragraph, DefaultTerminal};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let apprun = run(terminal);
    ratatui::restore();
    apprun
}


fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Bem vindo a super aplicação TUI! Aperte 'q' para sair!")
                .white()
                .on_black();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        };
    }
}
