use crate::codegen::types::{Trait, TraitType, Data, DataType, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use std::collections::HashMap;

fn wrapperfn_call<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    return args.get(0).unwrap().tp.wrapperfn.unwrap()(codegen, args[1..].to_vec(), pos);
}

fn wrapperfn_bool<'a>(codegen: &mut codegen::CodeGen<'a>, _args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {  
    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(codegen.inkwell_types.booltp.const_int(0, false))),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

pub fn init_wrapperfn(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Call.to_string(), builtin_types::create_trait_func(wrapperfn_call, 0, TraitType::Call, new_datatype(BasicDataType::Unknown, BasicDataType::Unknown.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new())));

    let tp: DataType = new_datatype(BasicDataType::WrapperFunc, BasicDataType::WrapperFunc.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(wrapperfn_bool, 1, TraitType::Bool, tp.clone()));
    
    codegen.datatypes.insert(BasicDataType::WrapperFunc.to_string(), tp.clone());

    builtin_types::add_simple_type(codegen, traits, BasicDataType::WrapperFunc, BasicDataType::WrapperFunc.to_string().as_str());
}