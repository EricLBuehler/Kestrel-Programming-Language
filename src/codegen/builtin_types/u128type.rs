use crate::codegen::types::{Trait, TraitType, Data, DataType, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

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

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::U128.to_string()).unwrap().clone(),
        owned: true,
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

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::U128.to_string()).unwrap().clone(),
        owned: true,
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
    

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::U128.to_string()).unwrap().clone(),
        owned: true,
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

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::U128.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u128_pos<'a>(_codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    return args.get(0).unwrap().clone();
}

fn u128_bool<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {  
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::NE, selfv, codegen.inkwell_types.i8tp.const_zero(), "u128bool");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u128_eq<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 Eq, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::EQ, selfv, otherv, "u128eq");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u128_lt<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 Lt, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::ULT, selfv, otherv, "u128lt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u128_gt<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 Gt, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::UGT, selfv, otherv, "u128gt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u128_le<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 Le, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::ULE, selfv, otherv, "u128lt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u128_ge<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::U128 {
        let fmt: String = format!("invalid types for u128 Ge, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::UGE, selfv, otherv, "u128gt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn u128_ne<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::U8 {
        let fmt: String = format!("invalid types for u128 Ne, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::NE, selfv, otherv, "u128ne");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

pub fn init_u128(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();

    let tp: DataType = new_datatype(BasicDataType::U128, BasicDataType::U128.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    codegen.datatypes.insert(BasicDataType::U128.to_string(), tp.clone());

    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait_func(u128_add, 2, TraitType::Add, tp.clone()));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait_func(u128_mul, 2, TraitType::Mul, tp.clone()));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait_func(u128_sub, 2, TraitType::Sub, tp.clone()));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait_func(u128_div, 2, TraitType::Div, tp.clone()));
    traits.insert(TraitType::Pos.to_string(), builtin_types::create_trait_func(u128_pos, 1, TraitType::Pos, tp.clone()));
    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(u128_bool, 1, TraitType::Bool, tp.clone()));
    traits.insert(TraitType::Eq.to_string(), builtin_types::create_trait_func(u128_eq, 2, TraitType::Eq, tp.clone()));
    traits.insert(TraitType::Ne.to_string(), builtin_types::create_trait_func(u128_ne, 2, TraitType::Ne, tp.clone()));
    traits.insert(TraitType::Gt.to_string(), builtin_types::create_trait_func(u128_gt, 2, TraitType::Gt, tp.clone()));
    traits.insert(TraitType::Lt.to_string(), builtin_types::create_trait_func(u128_lt, 2, TraitType::Lt, tp.clone()));
    traits.insert(TraitType::Ge.to_string(), builtin_types::create_trait_func(u128_ge, 2, TraitType::Ge, tp.clone()));
    traits.insert(TraitType::Le.to_string(), builtin_types::create_trait_func(u128_le, 2, TraitType::Le, tp.clone()));
    
    builtin_types::add_simple_type(codegen, traits, BasicDataType::U128, BasicDataType::U128.to_string().as_str());
}