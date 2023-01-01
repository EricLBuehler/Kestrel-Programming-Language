use crate::codegen::types::{Trait, TraitType, Data, DataType, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow_literal<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<u64>().is_err() {
        let fmt: String = format!("Invalid u64 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn u64_add<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U64 {
        let fmt: String = format!("invalid types for u64 +, got '{}' and '{}'.", BasicDataType::U64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_add(selfv, otherv, "u64sum");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::U64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u64_mul<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U64 {
        let fmt: String = format!("invalid types for u64 *, got '{}' and '{}'.", BasicDataType::U64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "u64mul");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::U64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u64_sub<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U64 {
        let fmt: String = format!("invalid types for u64 -, got '{}' and '{}'.", BasicDataType::U64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_sub(selfv, otherv, "u64sub");
    

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::U64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u64_div<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U64 {
        let fmt: String = format!("invalid types for u64 /, got '{}' and '{}'.", BasicDataType::U64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_unsigned_div(selfv, otherv, "u64div");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::U64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u64_pos<'a>(_codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    return args.get(0).unwrap().clone();
}

pub fn init_u64(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();

    let tp: DataType = new_datatype(BasicDataType::U64, BasicDataType::U64.to_string(), None, Vec::new(), Vec::new(), None, false, None, Vec::new());

    codegen.datatypes.insert(BasicDataType::U64.to_string(), tp.clone());

    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait(u64_add, 2, TraitType::Add, tp.clone()));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait(u64_mul, 2, TraitType::Mul, tp.clone()));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait(u64_sub, 2, TraitType::Sub, tp.clone()));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait(u64_div, 2, TraitType::Div, tp.clone()));
    traits.insert(TraitType::Pos.to_string(), builtin_types::create_trait(u64_pos, 1, TraitType::Pos, tp.clone()));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::U64, BasicDataType::U64.to_string().as_str());
}