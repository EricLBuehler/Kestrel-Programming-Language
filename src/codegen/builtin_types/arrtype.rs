use crate::codegen::types::{Trait, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use std::collections::HashMap;

pub fn init_array(codegen: &mut codegen::CodeGen) {
    let traits: HashMap<String, Trait> = HashMap::new();

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Array, BasicDataType::Array.to_string().as_str());
}