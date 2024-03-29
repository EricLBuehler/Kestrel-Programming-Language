use std::collections::HashMap;
use crate::codegen;
use crate::codegen::types::{Type, BasicDataType, Trait, TraitType, Data};
use super::types::DataType;
use super::types;

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
pub mod booltype;

pub mod f32type;
pub mod f64type;

pub mod voidtype;
pub mod functype;
pub mod arrtype;
pub mod structtype;
pub mod wrapperfntype;
pub mod enumtype;

pub mod structs;
pub mod enums;

pub fn add_simple_type<'a>(codegen: &mut codegen::CodeGen<'a>, traits: HashMap<String, Trait<'a>>, basictype: BasicDataType, name: &str){
    let tp = Type {
        traits,
        basictype,
    };

    codegen.cur_module.types.insert(String::from(name), tp);
}

pub fn create_trait_func<'a>(function: fn(&mut codegen::CodeGen<'a>, Vec<Data<'a>>, &crate::parser::Position) -> Data<'a>, nargs: usize, traittype: TraitType, rettp: DataType<'a>) -> Trait<'a>{
    Trait {
        nargs,
        function: Some(function),
        inkfunc: None,
        traittype,
        rettp: Some(rettp),
    }
}

pub fn create_trait_ink<'a>(ink: inkwell::values::PointerValue<'a>, nargs: usize, traittype: TraitType, rettp: DataType<'a>) -> Trait<'a>{
    Trait {
        nargs,
        function: None,
        inkfunc: Some(ink),
        traittype,
        rettp: Some(rettp),
    }
}

pub fn create_empty_trait<'a>() -> Trait<'a>{
    Trait {
        nargs: 0usize,
        function: None,
        inkfunc: None,
        traittype: TraitType::User,
        rettp: None,
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
    codegen.cur_module.datatypes.insert(String::from(types::BasicDataType::Unknown.to_string()), types::new_datatype(BasicDataType::Unknown, BasicDataType::Unknown.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new()));

    functype::init_func(codegen);
    wrapperfntype::init_wrapperfn(codegen);
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
    booltype::init_bool(codegen);

    f32type::init_f32(codegen);
    f64type::init_f64(codegen);

    voidtype::init_void(codegen);
    arrtype::init_array(codegen);
    structtype::init_struct(codegen);
    enumtype::init_enum(codegen);
}

pub fn init_traits(codegen: &mut codegen::CodeGen) {
    codegen.traits.insert(types::TraitType::Add.to_string(), types::TraitSignature {
         nargs: Some(2), trait_sig: None, name: String::from("add"), traittp: types::TraitMetatype::Builtin, vars: None,
         implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Sub.to_string(), types::TraitSignature {
         nargs: Some(2), trait_sig: None, name: String::from("sub"), traittp: types::TraitMetatype::Builtin, vars: None,
         implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Mul.to_string(), types::TraitSignature {
         nargs: Some(2), trait_sig: None, name: String::from("mul"), traittp: types::TraitMetatype::Builtin, vars: None,
         implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Div.to_string(), types::TraitSignature {
         nargs: Some(2), trait_sig: None, name: String::from("div"), traittp: types::TraitMetatype::Builtin, vars: None,
         implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Pos.to_string(), types::TraitSignature {
         nargs: Some(1), trait_sig: None, name: String::from("pos"), traittp: types::TraitMetatype::Builtin, vars: None,
         implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Neg.to_string(), types::TraitSignature {
            nargs: Some(1), trait_sig: None, name: String::from("neg"), traittp: types::TraitMetatype::Builtin, vars: None,
            implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Bool.to_string(), types::TraitSignature {
            nargs: Some(1), trait_sig: None, name: String::from("bool"), traittp: types::TraitMetatype::Builtin, vars: None,
            implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Eq.to_string(), types::TraitSignature {
            nargs: Some(2), trait_sig: None, name: String::from("eq"), traittp: types::TraitMetatype::Builtin, vars: None,
            implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Ne.to_string(), types::TraitSignature {
        nargs: Some(2), trait_sig: None, name: String::from("ne"), traittp: types::TraitMetatype::Builtin, vars: None,
        implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Gt.to_string(), types::TraitSignature {
            nargs: Some(2), trait_sig: None, name: String::from("gt"), traittp: types::TraitMetatype::Builtin, vars: None,
            implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Lt.to_string(), types::TraitSignature {
            nargs: Some(2), trait_sig: None, name: String::from("lt"), traittp: types::TraitMetatype::Builtin, vars: None,
            implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Ge.to_string(), types::TraitSignature {
            nargs: Some(2), trait_sig: None, name: String::from("ge"), traittp: types::TraitMetatype::Builtin, vars: None,
            implementations: std::collections::HashMap::new(),
        });
    codegen.traits.insert(types::TraitType::Le.to_string(), types::TraitSignature {
            nargs: Some(2), trait_sig: None, name: String::from("le"), traittp: types::TraitMetatype::Builtin, vars: None,
            implementations: std::collections::HashMap::new(),
        });
}

pub fn init_structs(codegen: &mut codegen::CodeGen) {
    structs::stringtype::init_string(codegen);
}

pub fn init_enums(codegen: &mut codegen::CodeGen) {
    enums::optionaltype::init_optional(codegen);
    enums::resulttype::init_result(codegen);
}