mod app;
mod server;
mod protocol;

use log::*;
use structopt::StructOpt;
use app::AquamarineToolkitOpt;

const PROTOCOL_VERSION: u32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Parsing args");
    let opt: AquamarineToolkitOpt = AquamarineToolkitOpt::from_args();

}
