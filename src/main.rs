mod app;
mod server;
mod plugins;

use log::*;
use structopt::StructOpt;
use app::AquamarineToolkit;

fn main() {
    pretty_env_logger::init();
    trace!("Parsing args");
    let opt: AquamarineToolkit = AquamarineToolkit::from_args();
    if opt.daemon {
        server::start_server(opt.bind);
    }

}
