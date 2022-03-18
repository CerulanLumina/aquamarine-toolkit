use aquamarine_toolkit_api::{ PluginDefinition };

pub fn load_plugins() -> Vec<PluginDefinition> {
    let plugins: Vec<PluginDefinition> = crate::plugins::load_internal_plugins().into_iter().chain(load_external_plugins().into_iter()).collect();

    plugins.iter().for_each(|a| (a.initialize)());

    plugins
}

fn load_external_plugins() -> Vec<PluginDefinition> {
    vec![

    ]
}


