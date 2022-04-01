mod app;
mod daemon;
mod plugins;

use log::*;
use structopt::StructOpt;
use app::AquamarineToolkit;
use crate::app::ClientCommand;

fn main() {
    pretty_env_logger::init();
    trace!("Parsing args");
    let opt: AquamarineToolkit = AquamarineToolkit::from_args();
    if opt.daemon {
        daemon::start_server(opt.bind);
    } else if opt.command.is_some() {



        match opt.command.unwrap() {
            ClientCommand::Browser { url } => {
                plugins::browser::send(url)
            }
        }
    }

}
