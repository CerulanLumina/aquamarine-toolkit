pub mod plugin_load;

use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aquamarine-toolkit", about = "A collection of tools for multi-computer productivity")]
pub struct AquamarineToolkit {
    #[structopt(long, short)]
    pub daemon: bool,

    #[structopt(long, required_if("daemon", "true"), default_value = "[::0]:7262")]
    pub bind: SocketAddr
}


