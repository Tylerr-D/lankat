use std::io::Read;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, UdpSocket};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub const PORT: u16 = 42424;

pub enum Event {
    PeerFound {
        name: String,
        addr: SocketAddrV4
    },
    WebMessage {
        text: String
    },
    TcpMessage {
        from: SocketAddrV4,
        text: String,
    }
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

    let tx_udp = tx.clone();
    thread::spawn(move || {
        let bind_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, PORT);
        let socket = UdpSocket::bind(bind_addr).expect("listen socket");

        let mut already = std::collections::HashSet::new();
        let mut buf = [0u8; 512];

        loop {
            let (n, addr) = socket.recv_from(&mut buf).expect("receive");

            let SocketAddr::V4(v4_addr) = addr else { continue };

            let Some(rest) = buf[..n].strip_prefix(b"lankat:") else { continue };

            let Ok(name) = std::str::from_utf8(rest) else { continue };

            if already.insert(v4_addr) {
                tx_udp.send(Event::PeerFound {
                    name: name.to_string(),
                    addr: v4_addr,
                }).expect("pass peer to ui");
            }
        }
    });

    let tx_tcp = tx.clone();
    thread::spawn(move || {
        let bind_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, PORT);
        let listener = TcpListener::bind(bind_addr).expect("tcp listener binder");

        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let Ok(peer_addr) = stream.peer_addr() else { continue };

            let SocketAddr::V4(v4_addr) = peer_addr else { continue };

            let tx_new = tx_tcp.clone();
            thread::spawn(move || {
                let mut buff = [0u8; 512];
                while let Ok(thing) = stream.read(&mut buff) {
                    if thing == 0 { break }

                    if let Ok(text) = std::str::from_utf8(&buff[..thing]) {
                        tx_new.send(Event::TcpMessage {
                            from: v4_addr,
                            text: text.to_string(),
                        }).ok();
                    }
                }
            });
        }
    });
}

// holy nested code, i think this is what is called bad code...

// lol
