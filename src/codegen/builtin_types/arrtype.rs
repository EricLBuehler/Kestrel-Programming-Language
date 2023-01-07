use crate::codegen::types::{Trait, BasicDataType, new_datatype, DataType, Data, Method, MethodType, TraitType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::errors;
use crate::parser;
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

fn array_bool<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    let res: inkwell::values::IntValue = if args.get(0).unwrap().tp.arrtp.unwrap().len()> 0 {codegen.inkwell_types.i8tp.const_int(1, false)} else {codegen.inkwell_types.i8tp.const_int(0, false)};

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::I8.to_string()).unwrap().clone(),
        owned: true,
    };
}

pub fn init_array(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    let mut methods: HashMap<String, Method> = HashMap::new();
    
    let tp: DataType = new_datatype(BasicDataType::Array, BasicDataType::Array.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(array_bool, 1, TraitType::Bool, tp.clone()));
    
    codegen.datatypes.insert(BasicDataType::Array.to_string(), tp);

    //length()
    let mut lengthfntp: DataType = codegen.datatypes.get(&BasicDataType::Func.to_string()).unwrap().clone();
    lengthfntp.name = String::from("length");
    lengthfntp.names = Some(vec![String::from("self")]);
    lengthfntp.rettp = Some(Box::new(codegen.datatypes.get("usize").unwrap().clone()));
    lengthfntp.types = vec![codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone()];

    methods.insert(String::from("length"), Method {
        tp: MethodType::Builtin,
        builtin: Some(length),
        func: None,
        functp: lengthfntp,
        isinstance: true,
    });

    let mut alt_tp: DataType = codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone();
    alt_tp.methods = methods;

    codegen.datatypes.insert(BasicDataType::Array.to_string(), alt_tp);


    builtin_types::add_simple_type(codegen, traits, BasicDataType::Array, BasicDataType::Array.to_string().as_str());
}