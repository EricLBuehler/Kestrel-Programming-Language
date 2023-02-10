use crate::codegen::types::{Trait, TraitType, Data, DataType, new_datatype, BasicDataType};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn check_overflow_literal<'a>(codegen: &codegen::CodeGen<'a>, data: &String, pos: &parser::Position) {
    if data.parse::<i8>().is_err() {
        let fmt: String = format!("Invalid i8 literal '{}'.", data);
        errors::raise_error(&fmt, errors::ErrorType::InvalidLiteralForRadix, pos, codegen.info);
    }
}

fn i8_add<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 +, got '{}' and '{}'.", BasicDataType::I8, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_add(selfv, otherv, "i8sum");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I8.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_mul<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 *, got '{}' and '{}'.", BasicDataType::I8, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "i8mul");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I8.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_sub<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 -, got '{}' and '{}'.", BasicDataType::I8, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_sub(selfv, otherv, "i8sub");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I8.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_div<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 /, got '{}' and '{}'.", BasicDataType::I8, args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = args.get(1).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_signed_div(selfv, otherv, "i8div");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I8.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_pos<'a>(_codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    return args.get(0).unwrap().clone();
}

fn i8_neg<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();
    let otherv: inkwell::values::IntValue = codegen.inkwell_types.i8tp.const_int_from_string("-1", inkwell::types::StringRadix::Decimal).unwrap();

    let res: inkwell::values::IntValue = codegen.builder.build_int_mul(selfv, otherv, "i8neg");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I8.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_bool<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {    
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::NE, selfv, codegen.inkwell_types.i32tp.const_zero(), "i8bool");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_eq<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 Eq, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::EQ, selfv, otherv, "i8eq");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_lt<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 Lt, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::SLT, selfv, otherv, "i8lt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_gt<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 Gt, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::SGT, selfv, otherv, "i8gt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_le<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 Le, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::SLE, selfv, otherv, "i8lt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_ge<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 Ge, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::SGE, selfv, otherv, "i8gt");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_ne<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {    
    if args.get(1).unwrap().tp != BasicDataType::I8 {
        let fmt: String = format!("invalid types for i8 Ne, got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();  
    let otherv: inkwell::values::IntValue = args.get(0).unwrap().data.unwrap().into_int_value();

    let res: inkwell::values::IntValue = codegen.builder.build_int_compare(inkwell::IntPredicate::NE, selfv, otherv, "i8ne");

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn i8_string<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    let selfv: inkwell::values::IntValue = args.first().unwrap().data.unwrap().into_int_value();

    //int sprintf(char* str, char* format)
    let sprintf: inkwell::values::FunctionValue = codegen.module.add_function("sprintf", codegen.inkwell_types.i32tp.fn_type(&[inkwell::types::BasicMetadataTypeEnum::PointerType(codegen.inkwell_types.i8tp.ptr_type(inkwell::AddressSpace::from(0u16))), inkwell::types::BasicMetadataTypeEnum::PointerType(codegen.inkwell_types.i8tp.ptr_type(inkwell::AddressSpace::from(0u16)))], true), Some(inkwell::module::Linkage::External));
    

    let struct_tp: inkwell::types::StructType = codegen.context.struct_type(&[inkwell::types::BasicTypeEnum::ArrayType(codegen.inkwell_types.i8tp.array_type(20))], false);

    let ptr: inkwell::values::PointerValue = codegen::CodeGen::alloca(codegen, struct_tp, "String");

    let arrptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(ptr, 0 as u32, "arr").expect("GEP Error");
    let data_ptr: inkwell::values::PointerValue = unsafe { codegen.builder.build_in_bounds_gep(arrptr, &[codegen.inkwell_types.i32tp.const_zero(), codegen.inkwell_types.i32tp.const_zero()], "data_ptr") };

        
    let arraytp: inkwell::types::ArrayType = codegen.inkwell_types.i8tp.array_type(3);

    let mut arrdata: Vec<inkwell::values::IntValue> = Vec::new();
    for c in b"%d" {
        arrdata.push(codegen.inkwell_types.i8tp.const_int(c.clone() as u64, false));
    }
    arrdata.push(codegen.inkwell_types.i8tp.const_zero());

    let intv: inkwell::values::IntValue = codegen.builder.build_int_z_extend(selfv, *codegen.inkwell_types.i32tp, "extend");

    let array: inkwell::values::ArrayValue = codegen.inkwell_types.i8tp.const_array(&arrdata[..]);
    let formatptr: inkwell::values::PointerValue = codegen::CodeGen::alloca(codegen, arraytp, "format");
    codegen.builder.build_store(formatptr, array);
    let format_ptr: inkwell::values::PointerValue = unsafe { codegen.builder.build_in_bounds_gep(formatptr, &[codegen.inkwell_types.i32tp.const_zero(), codegen.inkwell_types.i32tp.const_zero()], "data_ptr") };

    codegen.builder.build_call(inkwell::values::CallableValue::try_from(sprintf.as_global_value().as_pointer_value()).unwrap(), &[inkwell::values::BasicMetadataValueEnum::PointerValue(data_ptr), inkwell::values::BasicMetadataValueEnum::PointerValue(format_ptr), inkwell::values::BasicMetadataValueEnum::IntValue(intv)], "sprintf_call");

    let data: Data = Data {
        data: Some(inkwell::values::BasicValueEnum::PointerValue(ptr)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &String::from("String")).unwrap().clone(),
        owned: true,
    };
    return data;
}

pub fn init_i8(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();

    let mut tp: DataType = new_datatype(BasicDataType::I8, BasicDataType::I8.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    //to_string
    let mut tostrfntp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::WrapperFunc.to_string()).unwrap().clone();
    tostrfntp.name = BasicDataType::WrapperFunc.to_string();
    tostrfntp.names = Some(vec![String::from("self")]);
    tostrfntp.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, &crate::codegen::types::BasicDataType::I32.to_string()).unwrap().clone()));
    tostrfntp.wrapperfn = Some(i8_string);
    
    tp.methods.insert(String::from("to_string"), crate::codegen::types::Method {
        tp: crate::codegen::types::MethodType::Builtin,
        builtin: Some(i8_string),
        func: None,
        functp: tostrfntp,
        isinstance: true,
        isinstanceptr: false,
    });
    //
    codegen.cur_module.datatypes.insert(BasicDataType::I8.to_string(), tp.clone());       

    traits.insert(TraitType::Add.to_string(), builtin_types::create_trait_func(i8_add, 2, TraitType::Add, tp.clone()));
    traits.insert(TraitType::Mul.to_string(), builtin_types::create_trait_func(i8_mul, 2, TraitType::Mul, tp.clone()));
    traits.insert(TraitType::Sub.to_string(), builtin_types::create_trait_func(i8_sub, 2, TraitType::Sub, tp.clone()));
    traits.insert(TraitType::Div.to_string(), builtin_types::create_trait_func(i8_div, 2, TraitType::Div, tp.clone()));
    traits.insert(TraitType::Pos.to_string(), builtin_types::create_trait_func(i8_pos, 1, TraitType::Pos, tp.clone()));
    traits.insert(TraitType::Neg.to_string(), builtin_types::create_trait_func(i8_neg, 1, TraitType::Neg, tp.clone()));
    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(i8_bool, 1, TraitType::Bool, tp.clone()));
    traits.insert(TraitType::Eq.to_string(), builtin_types::create_trait_func(i8_eq, 2, TraitType::Eq, tp.clone()));
    traits.insert(TraitType::Ne.to_string(), builtin_types::create_trait_func(i8_ne, 2, TraitType::Ne, tp.clone()));
    traits.insert(TraitType::Gt.to_string(), builtin_types::create_trait_func(i8_gt, 2, TraitType::Gt, tp.clone()));
    traits.insert(TraitType::Lt.to_string(), builtin_types::create_trait_func(i8_lt, 2, TraitType::Lt, tp.clone()));
    traits.insert(TraitType::Ge.to_string(), builtin_types::create_trait_func(i8_ge, 2, TraitType::Ge, tp.clone()));
    traits.insert(TraitType::Le.to_string(), builtin_types::create_trait_func(i8_le, 2, TraitType::Le, tp.clone()));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::I8, BasicDataType::I8.to_string().as_str());
}