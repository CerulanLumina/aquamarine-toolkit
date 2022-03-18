
pub struct PluginDefinition {
    pub name: String,
    pub initialize: Box<dyn Fn()>,
    pub handler_function: Box<dyn Fn(Vec<u8>)>
}

pub enum AquamarineMessage {
    Signal {
        target: String,
        data: Vec<u8>
    }
}
