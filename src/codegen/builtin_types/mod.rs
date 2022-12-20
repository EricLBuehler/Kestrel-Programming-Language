use std::collections::HashMap;
use crate::codegen;
use crate::codegen::types::{Type, BasicDataType, Trait, TraitType, Data};

use super::types::DataType;

pub mod i32type;

pub mod unittype;
pub mod functype;

fn add_simple_type<'a>(codegen: &mut codegen::CodeGen<'a>, traits: HashMap<String, Trait<'a>>, basictype: BasicDataType, name: &str){
    let tp = Type {
        attributes: HashMap::new(),
        traits,
        methods: HashMap::new(),
        basictype,
    };

    codegen.types.insert(String::from(name), tp);
}

fn create_trait<'a>(function: fn(&codegen::CodeGen<'a>, Vec<Data<'a>>, &crate::parser::Position) -> Data<'a>, nargs: usize, traittype: TraitType, rettp: DataType) -> Trait<'a>{
    Trait {
        nargs,
        function,
        traittype,
        rettp,
    }
}

pub fn init(codegen: &mut codegen::CodeGen) {
    i32type::init_i32(codegen);

    unittype::init_unit(codegen);
    functype::init_func(codegen);
}