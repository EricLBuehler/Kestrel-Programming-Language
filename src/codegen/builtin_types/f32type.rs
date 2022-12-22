use crate::codegen::types::{Trait, TraitType, Data, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow_literal<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<f32>().is_err() {
        let fmt: String = format!("Invalid f32 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn f32_add<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f32 +, got '{}' and '{}'.", BasicDataType::F32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = args.get(1).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::FloatValue = codegen.builder.build_float_add(selfv, otherv, "f32sum");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: new_datatype(BasicDataType::F32, BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn f32_mul<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f32 *, got '{}' and '{}'.", BasicDataType::F32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = args.get(1).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::FloatValue = codegen.builder.build_float_mul(selfv, otherv, "f32mul");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: new_datatype(BasicDataType::F32, BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn f32_sub<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f32 -, got '{}' and '{}'.", BasicDataType::F32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = args.get(1).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::FloatValue = codegen.builder.build_float_sub(selfv, otherv, "f32sub");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: new_datatype(BasicDataType::F32, BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

fn f32_div<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f32 /, got '{}' and '{}'.", BasicDataType::F32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = args.get(1).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::FloatValue = codegen.builder.build_float_div(selfv, otherv, "f32div");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: new_datatype(BasicDataType::F32, BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None),
    };
}

pub fn init_f32(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait(f32_add, 2, TraitType::Add, new_datatype(BasicDataType::F32, BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait(f32_mul, 2, TraitType::Mul, new_datatype(BasicDataType::F32, BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait(f32_sub, 2, TraitType::Sub, new_datatype(BasicDataType::F32, BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None)));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait(f32_div, 2, TraitType::Div, new_datatype(BasicDataType::F32, BasicDataType::F32.to_string(), None, Vec::new(), Vec::new(), None)));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::F32, BasicDataType::F32.to_string().as_str());
}