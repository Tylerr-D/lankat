pub mod tui;
pub mod network;
pub mod webpage;

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen
        ,
        disable_raw_mode,
        enable_raw_mode,
    },
};
use std::{
    io,
    sync::mpsc,
    time::Duration,
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};

use tui::app::App;
use tui::ui;

use crate::webpage::web;
use crate::network::net;

fn main() -> io::Result<()> {
    web::start();
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
    let (tx, rx) = mpsc::channel();
    let name = std::env::var("USER").unwrap_or_else(|_| "me".to_string());

    net::start(name, tx);

    let mut app = App::default();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if let Ok(net::Event::PeerFound { name, addr }) = rx.try_recv() {
            app.peers.push(format!("{name} at {addr}"));
        }

        while event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && app.handle_key(key.code) {
                    return Ok(());
                }
            }
        }

    }
}

// this is bad code lol

// dw, i dont understand it so i couldnt tell the difference lmao
