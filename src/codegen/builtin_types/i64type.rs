use crate::codegen::types::{Trait, TraitType, Data, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow_literal<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<i64>().is_err() {
        let fmt: String = format!("Invalid i64 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn i64_add<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I64 {
        let fmt: String = format!("invalid types for i64 +, got '{}' and '{}'.", BasicDataType::I64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_add(selfv, otherv, "i64sum");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None),
        owned: true,
    };
}

fn i64_mul<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I64 {
        let fmt: String = format!("invalid types for i64 *, got '{}' and '{}'.", BasicDataType::I64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "i64mul");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None),
        owned: true,
    };
}

fn i64_sub<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I64 {
        let fmt: String = format!("invalid types for i64 -, got '{}' and '{}'.", BasicDataType::I64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_sub(selfv, otherv, "i64sub");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None),
        owned: true,
    };
}

fn i64_div<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I64 {
        let fmt: String = format!("invalid types for i64 /, got '{}' and '{}'.", BasicDataType::I64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_signed_div(selfv, otherv, "i64div");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None),
        owned: true,
    };
}

fn i64_pos<'a>(_codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    return args.get(0).unwrap().clone();
}

fn i64_neg<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = codegen.inkwell_types.i64tp.const_int_from_string("-1", inkwell::types::StringRadix::Decimal).unwrap();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "i64neg");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None),
        owned: true,
    };
}

pub fn init_i64(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait(i64_add, 2, TraitType::Add, new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None)));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait(i64_mul, 2, TraitType::Mul, new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None)));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait(i64_sub, 2, TraitType::Sub, new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None)));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait(i64_div, 2, TraitType::Div, new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None)));
    traits.insert(TraitType::Pos.to_string(), builtin_types::create_trait(i64_pos, 1, TraitType::Pos, new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None)));
    traits.insert(TraitType::Neg.to_string(), builtin_types::create_trait(i64_neg, 1, TraitType::Neg, new_datatype(BasicDataType::I64, BasicDataType::I64.to_string(), None, Vec::new(), Vec::new(), None, false, None)));
    
    builtin_types::add_simple_type(codegen, traits, BasicDataType::I64, BasicDataType::I64.to_string().as_str());
}