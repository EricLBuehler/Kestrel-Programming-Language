use crate::codegen::{CodeGen, Namespaces, ForwardDeclarationType};
use crate::codegen::modules::*;
use crate::codegen::types::*;

pub fn init_std(codegen: &mut CodeGen) {
    //std module
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

    let mut module: Module = Module { 
        name: String::from("std"),
        namespaces,
        modules: std::collections::HashMap::new(),
    };
    //

    //out module
    let mut out_functions = std::collections::HashMap::new();

    //printf
    let mut fntp: DataType = codegen.datatypes.get(&BasicDataType::Func.to_string()).unwrap().clone();
    fntp.name = String::from("printf");
    fntp.names = Some(vec![String::from("arg")]);
    fntp.rettp = Some(Box::new(codegen.datatypes.get(&BasicDataType::I32.to_string()).unwrap().clone()));
    fntp.types = vec![codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone()];

    out_functions.insert(String::from("printf"), (codegen.module.add_function("printf", codegen.inkwell_types.i32tp.fn_type(&[inkwell::types::BasicMetadataTypeEnum::PointerType(codegen.inkwell_types.i8tp.ptr_type(inkwell::AddressSpace::from(0u16)))], true), Some(inkwell::module::Linkage::External)), fntp, ForwardDeclarationType::Real));
    //
    let namespaces: Namespaces = Namespaces {
        locals: Vec::new(),
        functions: out_functions,
        structs: std::collections::HashMap::new(),
        template_functions_sig: std::collections::HashMap::new(),
        template_functions: Vec::new(),
        structid: std::collections::HashMap::new(),
        structid_from: std::collections::HashMap::new(),
        structid_max: -1,
        generic_enums: std::collections::HashMap::new(),
    };

    let out: Module = Module { 
        name: String::from("out"),
        namespaces,
        modules: std::collections::HashMap::new(),
    };
    module.modules.insert(out.name.clone(), out);
    //

    codegen.cur_module.modules.insert(module.name.clone(), module);
}