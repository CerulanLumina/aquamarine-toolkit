mod app;
mod daemon;
mod plugins;

use crate::app::ClientCommand;
use app::AquamarineToolkit;
use log::*;
use structopt::StructOpt;

fn main() {
    pretty_env_logger::init();
    trace!("Parsing args");
    let opt: AquamarineToolkit = AquamarineToolkit::from_args();
    if opt.daemon {
        daemon::start_server(opt.bind);
    } else if opt.command.is_some() {
        match opt.command.unwrap() {
            ClientCommand::Browser { url } => plugins::browser::send(url),
        }
    }
}
