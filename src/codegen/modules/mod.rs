pub struct Module<'a> {
    pub name: String,
    pub namespaces: crate::codegen::Namespaces<'a>,
    pub modules: Vec<Module<'a>>,
}