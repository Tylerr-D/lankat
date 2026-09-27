// module declaration
pub mod tui;
pub mod network;
pub mod webpage;

// Other library imports
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
    thread,
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};

// imports of us
use tui::app::App;
use tui::ui;

use crate::webpage::web;
use crate::network::net;
use crate::network::tcp;

// main
fn main() -> io::Result<()> {

    if std::env::args().any(|a| a == "-V" || a == "--version") {
        println!("lankat {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // starting whatever needs to be started
    let (tx, rx) = mpsc::channel();
    let (out_tx, out_rx) = tokio::sync::mpsc::channel(64);
    let info_tx = out_tx.clone();

    // getting user on a separate thread for some reason
    thread::spawn(move || {
        let name = std::env::var("USER").unwrap_or_else(|_| "me".to_string());
        loop {
            let _ = info_tx.try_send(format!("name: {name}"));
            thread::sleep(Duration::from_secs(1));
        }
    });

    // fallback
    let name = std::env::var("USER").unwrap_or_else(|_| "me".to_string());

    // starting the networks, opening ports, etc, and all the threads
    net::start(name.clone(), tx.clone());

    // starting hosting the website
    web::start(tx, out_tx.clone(), out_rx);

    // no clue wtf this does
    enable_raw_mode()?;

    // setting stdout as a variable
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let res = run(&mut terminal, rx, out_tx, name);

    // again, no clue wtf this does
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}

// dun dun dun
// run all the main loop
fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    rx: mpsc::Receiver<net::Event>,
    out_tx: tokio::sync::mpsc::Sender<String>,
    name: String,
) -> io::Result<()> {
    let mut app = App::default();
    let mut peer_ip: Vec<std::net::Ipv4Addr> = Vec::new();
    loop {
        // draws the tui
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // when new event
        while let Ok(ev) = rx.try_recv() {
            match ev {
                // if the event is a peer found
                net::Event::PeerFound { name, addr } => {
                    let peer_name = format!("{name} at {addr}");
                    if !app.peers.contains(&peer_name) {
                        app.peers.push(peer_name);
                        peer_ip.push(*addr.ip());
                    }
                }

                // if the event is a new message from the web
                net::Event::WebMessage { text } => {
                    app.messages.push(format!("web: {text}"));
                }

                // if the event is a new message from another lankat via tcp
                net::Event::TcpMessage { from, text } => {
                    let display = match serde_json::from_str::<tcp::TcpPacket>(&text){
                        Ok(p) => match p.get_payload(){
                            tcp::PacketType::Text(t) => format!("{}: {t}", p.get_sender()),
                            tcp::PacketType::Image {filename, .. } => {
                                format!("[image] {}: {filename}", p.get_sender())
                            }
                        },
                        Err(_) => format!("tcp: {from}, {text}"),
                    };
                    app.messages.push(display.clone());
                    let _ = out_tx.try_send(display);
                }
            }
        }

        // smth
        if event::poll(Duration::from_millis(10))? {
            let Event::Key(key) = event::read()? else { continue };

            // if key press, then change smth of the ui
            if key.kind != KeyEventKind::Press {
                continue;
            }

            // if key pressed is enter, then send message, else if smth, stop
            if key.code == KeyCode::Enter && !app.input.trim().is_empty() {
                let text = app.input.trim().to_string();
                app.handle_key(key.code);

                let _ = out_tx.try_send(text.clone());
                if let Some(&ip) = peer_ip.first(){
                    let _ = tcp::send_text(ip, name.clone(), text);
                }
            } else if app.handle_key(key.code) {
                return Ok(());
            }
        }
    }
}

// this is bad code lol

// dw, i dont understand it so i couldnt tell the difference lmao

// :sob: