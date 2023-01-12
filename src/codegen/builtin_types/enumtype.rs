use crate::codegen::types::{Trait, BasicDataType, new_datatype, DataType};
use crate::codegen;
use crate::codegen::builtin_types;
use std::collections::HashMap;

pub fn init_enum(codegen: &mut codegen::CodeGen) {
    let traits: HashMap<String, Trait> = HashMap::new();
    
    let tp: DataType = new_datatype(BasicDataType::Enum, BasicDataType::Enum.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    codegen.datatypes.insert(BasicDataType::Enum.to_string(), tp);

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Enum, BasicDataType::Enum.to_string().as_str());
}