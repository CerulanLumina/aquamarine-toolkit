use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum AquamarineToolkitOpt {
    Server {
        #[structopt(default_value = "0.0.0.0:7262", )]
        bind_ip: SocketAddr
    },
    Client {
        connect_ip: SocketAddr
    },
}
