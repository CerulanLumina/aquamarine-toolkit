use aquamarine_toolkit_api::PluginDefinition;
use log::{error, info};

const PLUGIN_NAME: &'static str = "aq-browser";

pub fn create_plugin() -> PluginDefinition {
    PluginDefinition {
        name: String::from(PLUGIN_NAME),
        initialize: Box::new(initialize),
        handler_function: Box::new(handler),
    }
}

fn initialize() {}

pub type URLType = String;

fn handler(data: Vec<u8>) {
    info!("Received Browser message");
    match bincode::deserialize::<URLType>(&data) {
        Ok(data) => {
            if let Err(err) = open::that(&data) {
                error!("Unable to open url {}. Error: {}", &data, err);
            }
        }
        Err(err) => {
            error!("Failed to deserialize message! {}", err);
        }
    }
}

pub fn send(url: URLType) {
    info!("Sending Browser message to daemon: \"{}\"", &url);
    if let Err(err) = aquamarine_toolkit_api::send_message_to_server(
        PLUGIN_NAME.into(),
        bincode::serialize(&url).unwrap(),
    ) {
        error!("Failed to send Browser message to host. Error: {}", err);
    }
}
