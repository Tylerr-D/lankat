use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub const PORT: u16 = 42424;

pub enum Event {
    PeerFound {
        name: String,
        addr: SocketAddr
    },
    WebMessage {
        text: String
    },
}


pub fn start(name: String, tx: Sender<Event>) {
    thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("announce socket");
        socket.set_broadcast(true).expect("enable broadcast");
        let packet = format!("lankat:{name}");

        loop {
            socket.send_to(packet.as_bytes(), ("255.255.255.255", PORT)).ok();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let socket = UdpSocket::bind(("0.0.0.0", PORT)).expect("listen socket");
        let mut already = std::collections::HashSet::new();
        let mut buf = [0u8; 512];

        loop {
            let (n, addr) = socket.recv_from(&mut buf).expect("receive");
            if let Some(rest) = buf[..n].strip_prefix(b"lankat:") {
                if let Ok(name) = std::str::from_utf8(rest) {
                    if already.insert(addr) {
                        tx.send(Event::PeerFound {
                            name: name.to_string(),
                            addr,
                        }).expect("pass peer to ui");
                    }
                }
            }
        }
    });
}