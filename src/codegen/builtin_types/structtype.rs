use crate::codegen::types::{Trait, BasicDataType, new_datatype, DataType};
use crate::codegen;
use crate::codegen::builtin_types;
use std::collections::HashMap;

pub fn init_struct(codegen: &mut codegen::CodeGen) {
    let traits: HashMap<String, Trait> = HashMap::new();
    
    let tp: DataType = new_datatype(BasicDataType::Struct, BasicDataType::Struct.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    codegen.cur_module.datatypes.insert(BasicDataType::Struct.to_string(), tp);

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Struct, BasicDataType::Struct.to_string().as_str());
}