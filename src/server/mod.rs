use std::collections::HashMap;
use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use log::{debug, error, info, trace, warn};
use aquamarine_toolkit_api::PluginDefinition;
use crate::plugins::AquamarineMessage;

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

    pub fn run_server(mut self, plugins_map: HashMap<String, PluginDefinition>) -> ! {
        let mut bytes = [0u8; 1024];
        loop {
            if let Ok(recv) = self.udp_socket.recv(&mut bytes) {
                let recv_slice = &bytes[0..recv];
                trace!("Received data: {:x?}", recv_slice);

                let message = bincode::deserialize::<AquamarineMessage>(recv_slice);
                match message {
                    Ok(message) => {
                        if let AquamarineMessage::Signal { target, data } = message {
                            if let Some(plugin) = plugins_map.get(&target) {
                                debug!("Found handler plugin {}", target);
                                (plugin.handler_function)(data);
                            }
                        }
                    },
                    Err(err) => {
                        debug!("Unable to deserialize incoming UDP message: {}", err);
                    }
                }

            } else {
                warn!("Failed to receive data from UDP");
            }
        }
    }
}

pub fn start_server(addr: SocketAddr) {

    let plugins = crate::app::plugin_load::load_plugins();
    let map = plugins.into_iter().map(|a| (a.name.clone(), a)).collect::<HashMap<_, _>>();

    match AquamarineServer::new(addr) {
        Ok(server) => {
            server.run_server(map)
        },
        Err(err) => {
            error!("Failed to start server {}", err);
        }
    }
}
