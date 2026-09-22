mod app;
mod ui;

use std::io;

use crossterm::{
    event::{self, Event, KeyEventkind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let res = run(&mut terminal);

    disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}

fn run (terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = app::App::default();
    loop {
        terminal.draw(|frame| ui::draw(fraame, &app))?;
        if let Event::key(key) = event::read()? {
            if key.kind == KeyEventkind::Press && app.handle_key(key.code) {
                breal;
            }
        }
    }
    Ok(())
}