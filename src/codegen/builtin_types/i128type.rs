use crate::codegen::types::{Trait, TraitType, Data, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<i128>().is_err() {
        let fmt: String = format!("Invalid i128 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn i128_add<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I128 {
        let fmt: String = format!("invalid types for i128 +, got '{}' and '{}'.", BasicDataType::I128, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_add(selfv, otherv, "i128sum");
    check_overflow(codegen, &res.to_string(), pos);

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I128, BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn i128_mul<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I128 {
        let fmt: String = format!("invalid types for i128 *, got '{}' and '{}'.", BasicDataType::I128, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "i128mul");
    check_overflow(codegen, &res.to_string(), pos);

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I128, BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn i128_sub<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I128 {
        let fmt: String = format!("invalid types for i128 -, got '{}' and '{}'.", BasicDataType::I128, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_sub(selfv, otherv, "i128sub");
    check_overflow(codegen, &res.to_string(), pos);

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I128, BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn i128_div<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I128 {
        let fmt: String = format!("invalid types for i128 /, got '{}' and '{}'.", BasicDataType::I128, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_signed_div(selfv, otherv, "i128div");
    check_overflow(codegen, &res.to_string(), pos);

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I128, BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

pub fn init_i128(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait(i128_add, 2, TraitType::Add, new_datatype(BasicDataType::I128, BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait(i128_mul, 2, TraitType::Mul, new_datatype(BasicDataType::I128, BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait(i128_sub, 2, TraitType::Sub, new_datatype(BasicDataType::I128, BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait(i128_div, 2, TraitType::Div, new_datatype(BasicDataType::I128, BasicDataType::I128.to_string(), None, Vec::new(), Vec::new(), None)));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::I128, BasicDataType::I128.to_string().as_str());
}