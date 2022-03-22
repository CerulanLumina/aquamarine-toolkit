use aquamarine_toolkit_api::PluginDefinition;

const PLUGIN_NAME: &'static str = "aq-browser";

pub fn create_plugin() -> PluginDefinition {
    PluginDefinition {
        name: String::from(PLUGIN_NAME),
        initialize: Box::new(initialize),
        handler_function: Box::new(handler),
    }
}

fn initialize() {

}

fn handler(data: Vec<u8>) {

}
