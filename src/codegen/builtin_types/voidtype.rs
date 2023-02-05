use crate::codegen::types::{Trait, BasicDataType, DataType, new_datatype, Data, TraitType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use std::collections::HashMap;

fn void_bool<'a>(codegen: &mut codegen::CodeGen<'a>, _args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {    
    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(codegen.inkwell_types.booltp.const_int(0, false))),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

pub fn init_void(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    
    let tp: DataType = new_datatype(BasicDataType::Void, BasicDataType::Void.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(void_bool, 1, TraitType::Bool, tp.clone()));
    
    codegen.datatypes.insert(BasicDataType::Void.to_string(), tp);

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Void, BasicDataType::Void.to_string().as_str());
}