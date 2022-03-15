use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use log::{debug, error, info};
use crate::protocol::{AquamarineMessage, ProtocolSender};

struct AquamarineServer {
    udp_socket: UdpSocket
}

impl AquamarineServer {
    pub fn new(addr: SocketAddr) -> Result<Self, Box<dyn Error>> {
        debug!("Binding to socket: {:?}", addr);
        let socket = UdpSocket::bind(addr);
        socket.map(|a| AquamarineServer { udp_socket: a }).map_err(|a| {
            error!("Failed to bind to socket {:?}", addr);
            a.into()
        })
    }

    pub fn run_server(mut self) -> ! {
        let mut bytes = [0u8; 1024];
        loop {
            if let Ok(recv) = self.udp_socket.recv(&mut bytes) {
                let recv_slice = &bytes[0..recv];
                
            }
        }
    }
}

pub fn start_server(addr: SocketAddr) {
    match AquamarineServer::new(addr) {
        Ok(server) => {
            server.run_server()
        },
        Err(err) => {
            error!("Failed to start server {}", err);
        }
    }
}
