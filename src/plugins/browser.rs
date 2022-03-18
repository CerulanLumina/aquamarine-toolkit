use aquamarine_toolkit_api::PluginDefinition;

pub fn create_plugin() -> PluginDefinition {
    PluginDefinition {
        name: String::from("aq-browser"),
        initialize: Box::new(initialize),
        handler_function: Box::new(handler)
    }
}

fn initialize() {

}

fn handler(data: Vec<u8>) {

}
