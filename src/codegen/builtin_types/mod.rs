use std::collections::HashMap;
use crate::codegen;
use crate::codegen::types::{Type, BasicDataType, Trait, TraitType, Data};
use super::types::DataType;

pub mod i32type;
pub mod u32type;
pub mod i8type;
pub mod u8type;
pub mod i16type;
pub mod u16type;
pub mod i64type;
pub mod u64type;
pub mod i128type;
pub mod u128type;

pub mod f32type;
pub mod f64type;

pub mod voidtype;
pub mod functype;
pub mod arrtype;
pub mod structtype;

fn add_simple_type<'a>(codegen: &mut codegen::CodeGen<'a>, traits: HashMap<String, Trait<'a>>, basictype: BasicDataType, name: &str){
    let tp = Type {
        traits,
        basictype,
    };

    codegen.types.insert(String::from(name), tp);
}

fn create_trait<'a>(function: fn(&codegen::CodeGen<'a>, Vec<Data<'a>>, &crate::parser::Position) -> Data<'a>, nargs: usize, traittype: TraitType, rettp: DataType<'a>) -> Trait<'a>{
    Trait {
        nargs,
        function,
        traittype,
        rettp,
    }
}

pub fn int_issigned(val: DataType) -> bool {
    match val.tp {
        BasicDataType::I8 |
        BasicDataType::I16 |
        BasicDataType::I32 |
        BasicDataType::I64 |
        BasicDataType::I128 => {
            return true;
        }
        _ => {
            return false;
        }        
    }
}

pub fn init(codegen: &mut codegen::CodeGen) {
    i32type::init_i32(codegen);
    u32type::init_u32(codegen);
    i8type::init_i8(codegen);
    u8type::init_u8(codegen);
    i16type::init_i16(codegen);
    u16type::init_u16(codegen);
    i64type::init_i64(codegen);
    u64type::init_u64(codegen);
    i128type::init_i128(codegen);
    u128type::init_u128(codegen);

    f32type::init_f32(codegen);
    f64type::init_f64(codegen);

    voidtype::init_void(codegen);
    functype::init_func(codegen);
    arrtype::init_array(codegen);
    structtype::init_struct(codegen);
}