use aquamarine_toolkit_api::PluginDefinition;
use serde::{Serialize, Deserialize};
mod browser;

pub fn load_internal_plugins() -> Vec<PluginDefinition> {
    vec![
        browser::create_plugin()
    ]
}

#[derive(Serialize, Deserialize)]
pub enum AquamarineMessage {
    Signal {
        target: String,
        data: Vec<u8>
    }
}

