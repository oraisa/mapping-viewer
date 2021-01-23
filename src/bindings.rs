
pub struct Window {
    pub tabs: Vec<Tab>,
    pub title: String
}

pub struct Tab {
    pub groups: Vec<BindingGroup>,
    pub title: String
}

pub struct BindingGroup {
    pub bindings: Vec<Binding>,
    pub title: String
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub keys: String,
    pub action: String,
}
