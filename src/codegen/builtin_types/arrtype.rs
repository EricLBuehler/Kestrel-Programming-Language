use crate::codegen::types::{Trait, BasicDataType, new_datatype, DataType};
use crate::codegen;
use crate::codegen::builtin_types;
use std::collections::HashMap;

pub fn init_array(codegen: &mut codegen::CodeGen) {
    let traits: HashMap<String, Trait> = HashMap::new();
    
    let tp: DataType = new_datatype(BasicDataType::Array, BasicDataType::Array.to_string(), None, Vec::new(), Vec::new(), None, false, None, Vec::new());

    codegen.datatypes.insert(BasicDataType::Array.to_string(), tp);

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Array, BasicDataType::Array.to_string().as_str());
}