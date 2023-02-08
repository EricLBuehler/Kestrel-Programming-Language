use crate::codegen::{CodeGen, Namespaces};
use crate::codegen::modules::*;

pub fn init_std(codegen: &mut CodeGen) {
    let namespaces: Namespaces = Namespaces {
        locals: Vec::new(),
        functions: std::collections::HashMap::new(),
        structs: std::collections::HashMap::new(),
        template_functions_sig: std::collections::HashMap::new(),
        template_functions: Vec::new(),
        structid: std::collections::HashMap::new(),
        structid_from: std::collections::HashMap::new(),
        structid_max: -1,
        generic_enums: std::collections::HashMap::new(),
    };

    let module: Module = Module { 
        name: String::from("std"),
        namespaces,
        modules: std::collections::HashMap::new(),
    };

    codegen.cur_module.modules.insert(module.name.clone(), module);
}