use aquamarine_toolkit_api::PluginDefinition;
use serde::{Serialize, Deserialize};
pub mod browser;

pub fn load_internal_plugins() -> Vec<PluginDefinition> {
    vec![
        browser::create_plugin()
    ]
}
