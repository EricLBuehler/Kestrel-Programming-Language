use crate::codegen::types::{Trait, TraitType, Data, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<u128>().is_err() {
        let fmt: String = format!("u128 overflow.");
        errors::raise_error(&fmt, errors::ErrorType::Overflow, pos, codegen.info);
    }
}

pub fn check_overflow_literal<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<u128>().is_err() {
        let fmt: String = format!("Invalid u128 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn u128_add<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 +, got '{}' and '{}'.", BasicDataType::U128, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_add(selfv, otherv, "u128sum");
    check_overflow(codegen, &res.to_string(), pos);

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn u128_mul<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 *, got '{}' and '{}'.", BasicDataType::U128, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "u128mul");
    check_overflow(codegen, &res.to_string(), pos);

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn u128_sub<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 -, got '{}' and '{}'.", BasicDataType::U128, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_sub(selfv, otherv, "u128sub");
    
    check_overflow(codegen, &res.to_string(), pos);

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn u128_div<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 /, got '{}' and '{}'.", BasicDataType::U128, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_unsigned_div(selfv, otherv, "u128div");
    check_overflow(codegen, &res.to_string(), pos);

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

pub fn init_u128(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait(u128_add, 2, TraitType::Add, new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait(u128_mul, 2, TraitType::Mul, new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait(u128_sub, 2, TraitType::Sub, new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait(u128_div, 2, TraitType::Div, new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None)));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::U128, BasicDataType::U128.to_string().as_str());
}