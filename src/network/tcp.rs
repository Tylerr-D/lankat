use std::{fs, io};
use serde::{ Serialize, Deserialize };
use base64::{ Engine, engine::general_purpose };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PacketType {
    Text(String),
    Image { filename: String, data: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TcpPacket {
    sender: String,
    payload: PacketType,
}

impl TcpPacket {
    pub fn new_text(sender: String, message: String) -> TcpPacket {
        Self { sender, payload: PacketType::Text(message) }
    }

    pub fn new_image(sender: String, file_name: &str) -> io::Result<TcpPacket> {
        let bytes = fs::read(file_name)?;
        let base64_data = general_purpose::STANDARD.encode(&bytes);

        let ext = std::path::Path::new(file_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png");

        let mime = match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            _ => "image/png",
        };

        let data_uri = format!("data:{mime};base64,{base64_data}");

        Ok(TcpPacket {
            sender,
            payload: PacketType::Image {
                filename: file_name.to_string(),
                data: data_uri,
            },
        })
    }

    pub fn get_sender(&self) -> String {
        self.sender.parse().unwrap()
    }

    pub fn get_payload(&self) -> PacketType {
        self.payload.clone()
    }
}
