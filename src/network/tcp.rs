use std::io;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

pub struct TcpPacket {
    ip: Ipv4Addr,
    port: u16,
    address: SocketAddrV4,
    header: String,
    message: String,
}

impl TcpPacket {
    pub fn new(ip: Ipv4Addr, port: u16, header: String, message: String) -> TcpPacket {
        let address = SocketAddrV4::new(ip, port);

        Self { ip, port, address, header, message }
    }

    pub fn get_ip(&self) -> Ipv4Addr {
        self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_address(&self) -> SocketAddrV4 {
        self.address
    }

    pub fn get_header(&self) -> String {
        self.header.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

pub(crate) fn send_message(tcp_packet: TcpPacket) -> io::Result<()> {
    let target_ip = tcp_packet.get_address();

    let mut stream = TcpStream::connect(target_ip)?;

    let payload = tcp_packet.message;
    stream.write_all(payload.as_ref())?;
    stream.flush()?;

    Ok(())
}
