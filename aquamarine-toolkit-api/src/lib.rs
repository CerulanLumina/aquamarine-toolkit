
pub struct PluginDefinition {
    pub name: String,
    pub initialize: Box<dyn Fn()>,
    pub handler_function: Box<dyn Fn(Vec<u8>)>
}
