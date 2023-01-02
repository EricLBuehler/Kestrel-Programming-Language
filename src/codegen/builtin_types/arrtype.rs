use crate::codegen::types::{Trait, BasicDataType, new_datatype, DataType, Data, Method, MethodType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::errors;
use std::collections::HashMap;

fn length<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=1 {
        let fmt: String = format!("Expected 1 argument, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    }

    let len: u32 = args.get(0).unwrap().tp.arrtp.unwrap().len();

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(codegen.inkwell_types.i32tp.const_int(len.into(), false))),
        tp: codegen.datatypes.get("usize").unwrap().clone(),
        owned: true,
    };
}

pub fn init_array(codegen: &mut codegen::CodeGen) {
    let traits: HashMap<String, Trait> = HashMap::new();
    let mut methods: Vec<Method> = Vec::new();
    
    let tp: DataType = new_datatype(BasicDataType::Array, BasicDataType::Array.to_string(), None, Vec::new(), Vec::new(), None, false, None, Vec::new());

    codegen.datatypes.insert(BasicDataType::Array.to_string(), tp);

    //length()
    let mut lengthfntp: DataType = codegen.datatypes.get(&BasicDataType::Func.to_string()).unwrap().clone();
    lengthfntp.name = String::from("length");
    lengthfntp.names = Some(vec![String::from("self")]);
    lengthfntp.rettp = Some(Box::new(codegen.datatypes.get("usize").unwrap().clone()));
    lengthfntp.types = vec![codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone()];

    methods.push(Method {
        tp: MethodType::Builtin,
        builtin: Some(length),
        func: None,
        functp: lengthfntp,
    });

    let mut alt_tp: DataType = codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone();
    alt_tp.methods = methods;

    codegen.datatypes.insert(BasicDataType::Array.to_string(), alt_tp);


    builtin_types::add_simple_type(codegen, traits, BasicDataType::Array, BasicDataType::Array.to_string().as_str());
}