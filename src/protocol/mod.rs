use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use log::error;

use serde::{Deserialize, Serialize};

pub struct ProtocolSender {
    inner_stream: TcpStream
}

impl ProtocolSender {
    pub fn from_stream(stream: TcpStream) -> Self {
        ProtocolSender { inner_stream: stream }
    }

    pub fn send_message(&mut self, message: AquamarineMessage) -> Result<(), Box<dyn Error>> {
        let bin = bincode::serialize(&message).expect("serialize fail?");
        let res = self.inner_stream.write_all(&bin).map_err(|a| a.into());
        if res.is_err() {
            error!("Failed to send message {}", res.as_ref().unwrap_err());
        }
        res
    }

}

#[derive(Serialize, Deserialize)]
pub enum AquamarineMessage {
    Hello {
        proto_version: u32
    },
    Signal()
}
