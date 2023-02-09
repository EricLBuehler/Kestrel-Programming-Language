use crate::codegen::{CodeGen, Namespaces, ForwardDeclarationType};
use crate::codegen::modules::*;
use crate::codegen::types::*;
use crate::errors;

macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

fn std_print<'a>(codegen: &mut CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if  args.get(0).unwrap().tp != crate::codegen::CodeGen::datatypes_get(codegen, &String::from("String")).unwrap().clone()  {
        let fmt: String = format!("invalid types for print, expected String, got '{}'.", args.get(0).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let ptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(args.get(0).unwrap().data.unwrap().into_pointer_value(), 0, "data").expect("GEP Error");
    let data_ptr: inkwell::values::PointerValue = unsafe { codegen.builder.build_in_bounds_gep(ptr, &[codegen.inkwell_types.i32tp.const_zero(), codegen.inkwell_types.i32tp.const_zero()], "data_ptr") };

    let res: inkwell::values::CallSiteValue = codegen.builder.build_call(inkwell::values::CallableValue::try_from(codegen.cur_module.modules.get("std").unwrap().namespaces.functions.get("printf").unwrap().0.as_global_value().as_pointer_value()).unwrap(), &[data_ptr.into()], "printf_call");

    let data: Data = Data {
        data: Some(res.try_as_basic_value().left().unwrap()),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I32.to_string()).unwrap().clone(),
        owned: true,
    };
    return data;
}

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
        types: std::collections::HashMap::new(),
        datatypes: std::collections::HashMap::new(),
        vtables: None,
        vtables_vec: Vec::new(),
    };
    //
    
    //printf
    let mut printftp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Func.to_string()).unwrap().clone();
    printftp.name = String::from("printf");
    printftp.names = Some(vec![String::from("arg")]);
    printftp.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I32.to_string()).unwrap().clone()));
    printftp.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone()];

    module.namespaces.functions.insert(String::from("printf"), (codegen.module.add_function("printf", codegen.inkwell_types.i32tp.fn_type(&[inkwell::types::BasicMetadataTypeEnum::PointerType(codegen.inkwell_types.i8tp.ptr_type(inkwell::AddressSpace::from(0u16)))], true), Some(inkwell::module::Linkage::External)), printftp.clone(), ForwardDeclarationType::Real));
    //

    //Add out
    let mut tp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Struct.to_string()).unwrap().clone();
    tp.name = String::from("out");
    tp.names = Some(vec![String::from("print")]);
    tp.types = vec![printftp.clone()];
    tp.mutability = vec![DataMutablility::Immutable];
    let mut methods: std::collections::HashMap<String, Method> = std::collections::HashMap::new();

    //print()
    let mut newfntype: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Func.to_string()).unwrap().clone();
    newfntype.name = String::from("print");
    newfntype.names = Some(vec![String::from("str")]);
    newfntype.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I32.to_string()).unwrap().clone()));
    
    let mut str_tp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &String::from("String")).unwrap().clone();
    str_tp.is_ref = true;
    newfntype.types = vec![str_tp];

    methods.insert(String::from("print"), Method {
        tp: MethodType::Builtin,
        builtin: Some(std_print),
        func: None,
        functp: newfntype,
        isinstance: false,
    });
    //
    tp.methods = methods;

    module.namespaces.structid.insert(String::from("out"), module.namespaces.structid_max);
    module.namespaces.structid_from.insert(module.namespaces.structid_max, String::from("out"));
    module.namespaces.structid_max += 1;

    module.datatypes.insert(String::from("out"), tp.clone());
    module.namespaces.structs.insert(String::from("out"), (tp, Some(inkwell::types::AnyTypeEnum::StructType(codegen.context.struct_type(&[], false))), hashmap!{String::from("print") => 0}, ForwardDeclarationType::Real));
    //

    codegen.cur_module.modules.insert(module.name.clone(), module);
}