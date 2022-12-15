use crate::codegen::types::{DataType, Trait};
use crate::codegen;
use crate::codegen::builtin_types;
use std::collections::HashMap;

pub fn init_unit(codegen: &mut codegen::CodeGen) {
    let traits: HashMap<String, Trait> = HashMap::new();

    builtin_types::add_simple_type(codegen, traits, DataType::Unit, "Unit");
}