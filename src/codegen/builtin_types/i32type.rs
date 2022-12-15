use crate::codegen::types::{DataType, Trait, TraitType, Data};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

fn i32_add<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<&Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != DataType::I32 {
        let fmt: String = format!("invalid types for i32 +, got {} and {}", DataType::I32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    return Data {
        data: Some(inkwell::values::AnyValueEnum::IntValue(codegen.builder.build_int_add(selfv, otherv, "i32sum"))),
        tp: DataType::I32,
    };
}

fn i32_mul<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<&Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != DataType::I32 {
        let fmt: String = format!("invalid types for i32 *, got {} and {}", DataType::I32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    return Data {
        data: Some(inkwell::values::AnyValueEnum::IntValue(codegen.builder.build_int_mul(selfv, otherv, "i32sum"))),
        tp: DataType::I32,
    };
}

fn i32_sub<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<&Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != DataType::I32 {
        let fmt: String = format!("invalid types for i32 -, got {} and {}", DataType::I32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    return Data {
        data: Some(inkwell::values::AnyValueEnum::IntValue(codegen.builder.build_int_sub(selfv, otherv, "i32sum"))),
        tp: DataType::I32,
    };
}

fn i32_div<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<&Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != DataType::I32 {
        let fmt: String = format!("invalid types for i32 /, got {} and {}", DataType::I32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    return Data {
        data: Some(inkwell::values::AnyValueEnum::IntValue(codegen.builder.build_int_signed_div(selfv, otherv, "i32sum"))),
        tp: DataType::I32,
    };
}

pub fn init_i32(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait(i32_add, 2, TraitType::Add, String::from("i32"), DataType::I32));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait(i32_mul, 2, TraitType::Mul, String::from("i32"), DataType::I32));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait(i32_sub, 2, TraitType::Sub, String::from("i32"), DataType::I32));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait(i32_div, 2, TraitType::Div, String::from("i32"), DataType::I32));

    builtin_types::add_simple_type(codegen, traits, DataType::I32, "i32");
}