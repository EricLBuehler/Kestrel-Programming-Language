use crate::codegen::types::{Trait, BasicDataType, new_datatype, DataType, Data, TraitType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::errors;
use crate::parser;
use std::collections::HashMap;

fn enum_eq<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != args.first().unwrap().tp {
        let fmt: String = format!("invalid types for enum Eq, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::EQ, selfv, otherv, "enum_eq");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn enum_ne<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != args.first().unwrap().tp {
        let fmt: String = format!("invalid types for enum Eq, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::NE, selfv, otherv, "enum_ne");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

pub fn init_enum(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    
    let tp: DataType = new_datatype(BasicDataType::Enum, BasicDataType::Enum.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    codegen.cur_module.datatypes.insert(BasicDataType::Enum.to_string(), tp.clone());

    traits.insert(TraitType::Eq.to_string(), builtin_types::create_trait_func(enum_eq, 2, TraitType::Eq, tp.clone()));
    traits.insert(TraitType::Ne.to_string(), builtin_types::create_trait_func(enum_ne, 2, TraitType::Ne, tp.clone()));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Enum, BasicDataType::Enum.to_string().as_str());
}