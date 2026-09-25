pub mod tui;
pub mod network;
pub mod webpage;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
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

    let (tx, rx) = mpsc::channel();
    let (out_tx, out_rx) = tokio::sync::mpsc::channel(64);
    let name = std::env::var("USER").unwrap_or_else(|_| "me".to_string());

    net::start(name, tx.clone());
    web::start(tx, out_rx);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let res = run(&mut terminal, rx, out_tx);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}

fn run (
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    rx: mpsc::Receiver<net::Event>,
    out_tx: tokio::sync::mpsc::Sender<String>,
) -> io::Result<()> {
    let mut app = App::default();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        while let Ok(ev) = rx.try_recv() {
            match ev {
                net::Event::PeerFound { name, addr } => {
                    let peer_name = format!("{} at {}", name, addr);
                    if !app.peers.contains(&peer_name) {
                        app.peers.push(peer_name);
                    }
                }
                net::Event::WebMessage { text } => {
                    app.messages.push(format!("web: {text}"));
                }
            }
        }

        if event::poll(Duration::from_millis(50))? {
            let Event::Key(key) = event::read()? else { continue };

            if key.kind != KeyEventKind::Press {
                continue
            }

            if key.code == KeyCode::Enter && !app.input.trim().is_empty() {
                let text = app.input.trim().to_string();
                app.handle_key(key.code);

                let _ = out_tx.try_send(text);
            } else if app.handle_key(key.code) {
                return Ok(());
            }
        }
    }
}

// this is bad code lol

// dw, i dont understand it so i couldnt tell the difference lmao

// :sob: