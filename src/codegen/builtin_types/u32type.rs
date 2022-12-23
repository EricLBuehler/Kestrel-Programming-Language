use crate::codegen::types::{Trait, TraitType, Data, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow_literal<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<u32>().is_err() {
        let fmt: String = format!("Invalid u32 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn u32_add<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U32 {
        let fmt: String = format!("invalid types for u32 +, got '{}' and '{}'.", BasicDataType::U32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_add(selfv, otherv, "u32sum");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false),
    };
}

fn u32_mul<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U32 {
        let fmt: String = format!("invalid types for u32 *, got '{}' and '{}'.", BasicDataType::U32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "u32mul");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false),
    };
}

fn u32_sub<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U32 {
        let fmt: String = format!("invalid types for u32 -, got '{}' and '{}'.", BasicDataType::U32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_sub(selfv, otherv, "u32sub");
    

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false),
    };
}

fn u32_div<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::U32 {
        let fmt: String = format!("invalid types for u32 /, got '{}' and '{}'.", BasicDataType::U32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_unsigned_div(selfv, otherv, "u32div");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false),
    };
}

fn u32_pos<'a>(_codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    return args.get(0).unwrap().clone();
}

pub fn init_u32(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait(u32_add, 2, TraitType::Add, new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false)));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait(u32_mul, 2, TraitType::Mul, new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false)));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait(u32_sub, 2, TraitType::Sub, new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false)));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait(u32_div, 2, TraitType::Div, new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false)));
    traits.insert(TraitType::Pos.to_string(), builtin_types::create_trait(u32_pos, 1, TraitType::Pos, new_datatype(BasicDataType::U32, BasicDataType::U32.to_string(), None, Vec::new(), Vec::new(), None, false)));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::U32, BasicDataType::U32.to_string().as_str());
}