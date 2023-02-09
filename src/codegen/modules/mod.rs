pub mod builtin_modules;

#[derive(Clone, Debug)]
pub struct Module<'ctx> {
    pub name: String,
    pub namespaces: crate::codegen::Namespaces<'ctx>,
    pub modules: std::collections::HashMap<String, Module<'ctx>>,
    pub vtables: Option<inkwell::values::GlobalValue<'ctx>>,
    pub vtables_vec: Vec<Vec<inkwell::values::PointerValue<'ctx>>>,
    pub datatypes: std::collections::HashMap<String, crate::codegen::types::DataType<'ctx>>,
    pub types: std::collections::HashMap<String, crate::codegen::types::Type<'ctx>>,
}