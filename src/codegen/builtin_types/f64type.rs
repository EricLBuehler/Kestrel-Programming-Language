use crate::codegen::types::{Trait, TraitType, Data, DataType, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow_literal<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<f64>().is_err() {
        let fmt: String = format!("Invalid f64 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn f64_add<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::F64 {
        let fmt: String = format!("invalid types for f64 +, got '{}' and '{}'.", BasicDataType::F64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = args.get(1).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::FloatValue = codegen.builder.build_float_add(selfv, otherv, "f64sum");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::F64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_mul<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::F64 {
        let fmt: String = format!("invalid types for f64 *, got '{}' and '{}'.", BasicDataType::F64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = args.get(1).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::FloatValue = codegen.builder.build_float_mul(selfv, otherv, "f64mul");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::F64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_sub<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::F64 {
        let fmt: String = format!("invalid types for f64 -, got '{}' and '{}'.", BasicDataType::F64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = args.get(1).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::FloatValue = codegen.builder.build_float_sub(selfv, otherv, "f64sub");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::F64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_div<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::F64 {
        let fmt: String = format!("invalid types for f64 /, got '{}' and '{}'.", BasicDataType::F64, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = args.get(1).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::FloatValue = codegen.builder.build_float_div(selfv, otherv, "f64div");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::F64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_pos<'a>(_codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    return args.get(0).unwrap().clone();
}

fn f64_neg<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();
    let otherv: inkwell::values::FloatValue = codegen.inkwell_types.f64tp.const_float_from_string("-1");

    let res: inkwell::values::FloatValue = codegen.builder.build_float_mul(selfv, otherv, "f64neg");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::FloatValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::F64.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_bool<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {    
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::IntValue = codegen.builder.build_float_compare(inkwell::FloatPredicate::ONE, selfv, codegen.inkwell_types.f64tp.const_zero(), "f64bool");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_eq<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f64 Eq, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();  
    let otherv: inkwell::values::FloatValue = args.get(0).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::IntValue = codegen.builder.build_float_compare(inkwell::FloatPredicate::OEQ, selfv, otherv, "f64eq");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_lt<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f64 Lt, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();  
    let otherv: inkwell::values::FloatValue = args.get(0).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::IntValue = codegen.builder.build_float_compare(inkwell::FloatPredicate::OLT, selfv, otherv, "f64lt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_gt<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f64 Gt, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();  
    let otherv: inkwell::values::FloatValue = args.get(0).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::IntValue = codegen.builder.build_float_compare(inkwell::FloatPredicate::OGT, selfv, otherv, "f64gt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_le<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f64 Le, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();  
    let otherv: inkwell::values::FloatValue = args.get(0).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::IntValue = codegen.builder.build_float_compare(inkwell::FloatPredicate::OLE, selfv, otherv, "f64lt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_ge<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::F32 {
        let fmt: String = format!("invalid types for f64 Ge, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();  
    let otherv: inkwell::values::FloatValue = args.get(0).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::IntValue = codegen.builder.build_float_compare(inkwell::FloatPredicate::OGE, selfv, otherv, "f64gt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_ne<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::F64 {
        let fmt: String = format!("invalid types for f64 Ne, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();  
    let otherv: inkwell::values::FloatValue = args.get(0).unwrap().data.unwrap().into_float_value();

    let res: inkwell::values::IntValue = codegen.builder.build_float_compare(inkwell::FloatPredicate::ONE, selfv, otherv, "f32eq");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn f64_string<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    let selfv: inkwell::values::FloatValue = args.first().unwrap().data.unwrap().into_float_value();

    //int sprintf(char* str, char* format)
    let sprintf: inkwell::values::FunctionValue = codegen.module.add_function("sprintf", codegen.inkwell_types.i32tp.fn_type(&[inkwell::types::BasicMetadataTypeEnum::PointerType(codegen.inkwell_types.i8tp.ptr_type(inkwell::AddressSpace::from(0u16))), inkwell::types::BasicMetadataTypeEnum::PointerType(codegen.inkwell_types.i8tp.ptr_type(inkwell::AddressSpace::from(0u16)))], true), Some(inkwell::module::Linkage::External));
    

    let struct_tp: inkwell::types::StructType = codegen.context.struct_type(&[inkwell::types::BasicTypeEnum::ArrayType(codegen.inkwell_types.i8tp.array_type(20))], false);

    let ptr: inkwell::values::PointerValue = codegen::CodeGen::alloca(codegen, struct_tp, "String");

    let arrptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(ptr, 0 as u32, "arr").expect("GEP Error");
    let data_ptr: inkwell::values::PointerValue = unsafe { codegen.builder.build_in_bounds_gep(arrptr, &[codegen.inkwell_types.i32tp.const_zero(), codegen.inkwell_types.i32tp.const_zero()], "data_ptr") };

        
    let arraytp: inkwell::types::ArrayType = codegen.inkwell_types.i8tp.array_type(3);

    let mut arrdata: Vec<inkwell::values::IntValue> = Vec::new();
    for c in b"%f" {
        arrdata.push(codegen.inkwell_types.i8tp.const_int(c.clone() as u64, false));
    }
    arrdata.push(codegen.inkwell_types.i8tp.const_zero());

    let array: inkwell::values::ArrayValue = codegen.inkwell_types.i8tp.const_array(&arrdata[..]);
    let formatptr: inkwell::values::PointerValue = codegen::CodeGen::alloca(codegen, arraytp, "format");
    codegen.builder.build_store(formatptr, array);
    let format_ptr: inkwell::values::PointerValue = unsafe { codegen.builder.build_in_bounds_gep(formatptr, &[codegen.inkwell_types.i32tp.const_zero(), codegen.inkwell_types.i32tp.const_zero()], "data_ptr") };

    codegen.builder.build_call(inkwell::values::CallableValue::try_from(sprintf.as_global_value().as_pointer_value()).unwrap(), &[inkwell::values::BasicMetadataValueEnum::PointerValue(data_ptr), inkwell::values::BasicMetadataValueEnum::PointerValue(format_ptr), inkwell::values::BasicMetadataValueEnum::FloatValue(selfv)], "sprintf_call");

    let data: Data = Data {
        data: Some(inkwell::values::BasicValueEnum::PointerValue(ptr)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &String::from("String")).unwrap().clone(),
        owned: true,
    };
    return data;
}

pub fn init_f64(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();

    let mut tp: DataType = new_datatype(BasicDataType::F64, BasicDataType::F64.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    //to_string
    let mut tostrfntp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::WrapperFunc.to_string()).unwrap().clone();
    tostrfntp.names = Some(vec![String::from("self")]);
    tostrfntp.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, &crate::codegen::types::BasicDataType::I32.to_string()).unwrap().clone()));
    tostrfntp.wrapperfn = Some(f64_string);
    
    tp.methods.insert(String::from("to_string"), crate::codegen::types::Method {
        tp: crate::codegen::types::MethodType::Builtin,
        builtin: Some(f64_string),
        func: None,
        functp: tostrfntp,
        isinstance: true,
        isinstanceptr: false,
        ismutinstanceptr: false,
    });
    //

    codegen.cur_module.datatypes.insert(BasicDataType::F64.to_string(), tp.clone());

    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait_func(f64_add, 2, TraitType::Add, tp.clone()));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait_func(f64_mul, 2, TraitType::Mul, tp.clone()));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait_func(f64_sub, 2, TraitType::Sub, tp.clone()));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait_func(f64_div, 2, TraitType::Div, tp.clone()));
    traits.insert(TraitType::Pos.to_string(), builtin_types::create_trait_func(f64_pos, 1, TraitType::Pos, tp.clone()));
    traits.insert(TraitType::Neg.to_string(), builtin_types::create_trait_func(f64_neg, 1, TraitType::Neg, tp.clone()));
    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(f64_bool, 1, TraitType::Bool, tp.clone()));
    traits.insert(TraitType::Eq.to_string(), builtin_types::create_trait_func(f64_eq, 2, TraitType::Eq, tp.clone()));
    traits.insert(TraitType::Ne.to_string(), builtin_types::create_trait_func(f64_ne, 2, TraitType::Ne, tp.clone()));
    traits.insert(TraitType::Gt.to_string(), builtin_types::create_trait_func(f64_gt, 2, TraitType::Gt, tp.clone()));
    traits.insert(TraitType::Lt.to_string(), builtin_types::create_trait_func(f64_lt, 2, TraitType::Lt, tp.clone()));
    traits.insert(TraitType::Ge.to_string(), builtin_types::create_trait_func(f64_ge, 2, TraitType::Ge, tp.clone()));
    traits.insert(TraitType::Le.to_string(), builtin_types::create_trait_func(f64_le, 2, TraitType::Le, tp.clone()));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::F64, BasicDataType::F64.to_string().as_str());
}