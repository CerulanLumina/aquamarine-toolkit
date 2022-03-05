mod app;
mod server;

use log::*;
use structopt::StructOpt;
use app::AquamarineToolkitOpt;

fn main() {
    pretty_env_logger::init();
    trace!("Parsing args");
    let opt: AquamarineToolkitOpt = AquamarineToolkitOpt::from_args();

}
