pub mod builtin_modules;

pub struct Module<'a> {
    pub name: String,
    pub namespaces: crate::codegen::Namespaces<'a>,
    pub modules: std::collections::HashMap<String, Module<'a>>,
}