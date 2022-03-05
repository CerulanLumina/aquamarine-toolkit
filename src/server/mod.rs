use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use log::{debug, error, info};

struct AquamarineServer {
    tcp_listener: TcpListener
}

impl AquamarineServer {
    pub fn new(addr: SocketAddr) -> Result<Self, Box<dyn Error>> {
        debug!("Binding to socket: {:?}", addr);
        let listener = TcpListener::bind(addr);
        listener.map(|a| AquamarineServer { tcp_listener: a }).map_err(|a| {
            error!("Failed to bind to socket {:?}", addr);
            a.into()
        })
    }

    pub fn run_server(mut self) -> ! {
        loop {
            match self.tcp_listener.accept() {
                Ok((stream, addr)) => {
                    info!("Accepted connection from {}", addr);

                    handle_connection(stream)

                }
                Err(err) => {
                    debug!("Accepting incoming conn: {:?}", err);
                    error!("Failed to accept incoming connection. {}", err);
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {

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
