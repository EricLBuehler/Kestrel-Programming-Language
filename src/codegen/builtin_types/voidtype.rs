use crate::codegen::types::{Trait, BasicDataType, DataType, new_datatype};
use crate::codegen;
use crate::codegen::builtin_types;
use std::collections::HashMap;

pub fn init_void(codegen: &mut codegen::CodeGen) {
    let traits: HashMap<String, Trait> = HashMap::new();
    
    let tp: DataType = new_datatype(BasicDataType::Void, BasicDataType::Void.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    codegen.datatypes.insert(BasicDataType::Void.to_string(), tp);

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Void, BasicDataType::Void.to_string().as_str());
}