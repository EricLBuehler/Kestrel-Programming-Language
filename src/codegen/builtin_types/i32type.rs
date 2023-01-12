use crate::codegen::types::{Trait, TraitType, Data, DataType, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow_literal<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<i32>().is_err() {
        let fmt: String = format!("Invalid i32 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn i32_add<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 +, got '{}' and '{}'.", BasicDataType::I32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_add(selfv, otherv, "i32sum");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::I32.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_mul<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 *, got '{}' and '{}'.", BasicDataType::I32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "i32mul");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::I32.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_sub<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 -, got '{}' and '{}'.", BasicDataType::I32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_sub(selfv, otherv, "i32sub");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::I32.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_div<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 /, got '{}' and '{}'.", BasicDataType::I32, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_signed_div(selfv, otherv, "i32div");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::I32.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_pos<'a>(_codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    return args.get(0).unwrap().clone();
}

fn i32_neg<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = codegen.inkwell_types.i32tp.const_int_from_string("-1", inkwell::types::StringRadix::Decimal).unwrap();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "i32neg");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::I32.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_bool<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {      
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::NE, selfv, codegen.inkwell_types.i8tp.const_zero(), "i32bool");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_eq<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 Eq, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::EQ, selfv, otherv, "i32eq");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_lt<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 Lt, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::SGE, selfv, otherv, "i32lt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_gt<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 Gt, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::SGE, selfv, otherv, "i32gt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_le<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 Le, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::SGE, selfv, otherv, "i32lt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_ge<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 Ge, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::SGE, selfv, otherv, "i32gt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i32_ne<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I32 {
        let fmt: String = format!("invalid types for i32 Ne, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::NE, selfv, otherv, "i32ne");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

pub fn init_i32(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();

    let tp: DataType = new_datatype(BasicDataType::I32, BasicDataType::I32.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    codegen.datatypes.insert(BasicDataType::I32.to_string(), tp.clone());
    if std::mem::size_of::<isize>() == std::mem::size_of::<i32>() {
        codegen.datatypes.insert(String::from("isize"), tp.clone()); //Alias        
    }

    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait_func(i32_add, 2, TraitType::Add, tp.clone()));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait_func(i32_mul, 2, TraitType::Mul, tp.clone()));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait_func(i32_sub, 2, TraitType::Sub, tp.clone()));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait_func(i32_div, 2, TraitType::Div, tp.clone()));
    traits.insert(TraitType::Pos.to_string(), builtin_types::create_trait_func(i32_pos, 1, TraitType::Pos, tp.clone()));
    traits.insert(TraitType::Neg.to_string(), builtin_types::create_trait_func(i32_neg, 1, TraitType::Neg, tp.clone()));
    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(i32_bool, 1, TraitType::Bool, tp.clone()));
    traits.insert(TraitType::Eq.to_string(), builtin_types::create_trait_func(i32_eq, 2, TraitType::Eq, tp.clone()));
    traits.insert(TraitType::Ne.to_string(), builtin_types::create_trait_func(i32_ne, 2, TraitType::Ne, tp.clone()));
    traits.insert(TraitType::Gt.to_string(), builtin_types::create_trait_func(i32_gt, 2, TraitType::Gt, tp.clone()));
    traits.insert(TraitType::Lt.to_string(), builtin_types::create_trait_func(i32_lt, 2, TraitType::Lt, tp.clone()));
    traits.insert(TraitType::Ge.to_string(), builtin_types::create_trait_func(i32_ge, 2, TraitType::Ge, tp.clone()));
    traits.insert(TraitType::Le.to_string(), builtin_types::create_trait_func(i32_le, 2, TraitType::Le, tp.clone()));

    
    builtin_types::add_simple_type(codegen, traits, BasicDataType::I32, BasicDataType::I32.to_string().as_str());
}