use crate::codegen::types::{Trait, BasicDataType, new_datatype, DataType, Data, Method, MethodType, TraitType};
use crate::codegen::{self, CodeGen};
use crate::codegen::builtin_types;
use crate::errors;
use crate::parser;
use std::collections::HashMap;
use crate::codegen::builtin_types::enums;

fn arr_length<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=1 {
        let fmt: String = format!("Expected 1 argument, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    }

    let len: u32 = args.get(0).unwrap().tp.arrtp.unwrap().len();

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(if std::mem::size_of::<usize>() == std::mem::size_of::<u32>() {
            codegen.inkwell_types.i32tp.const_int(len.into(), false)
        }
        else {
            codegen.inkwell_types.i64tp.const_int(len.into(), false)
        })),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, "usize").unwrap().clone(),
        owned: true,
    };
}

fn array_bool<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    let res: inkwell::values::IntValue = if args.get(0).unwrap().tp.arrtp.unwrap().len()> 0 {codegen.inkwell_types.i8tp.const_int(1, false)} else {codegen.inkwell_types.i8tp.const_int(0, false)};

    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(res)),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn array_get<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=2 {
        let fmt: String = format!("Expected 2 arguments, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    } 

    if args.get(1).unwrap().tp != crate::codegen::CodeGen::datatypes_get(codegen, &String::from("usize")).unwrap() {
        let fmt: String = format!("Invalid types for Array.get, expected 'usize', got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    if args.get(1).unwrap().data.unwrap().into_int_value().is_const() && args.get(1).unwrap().data.unwrap().into_int_value().get_zero_extended_constant().unwrap() > args.get(0).unwrap().tp.arrtp.unwrap().len() as u64-1 {
        let fmt: String = format!("Array.get out of range. Maximum index is '{}'.", args.get(0).unwrap().tp.arrtp.unwrap().len());
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let mut opt: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &String::from("Optional")).unwrap().clone();
    opt.types = vec![args.first().unwrap().tp.types.first().unwrap().clone(), crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I32.to_string()).unwrap().clone()];

    let len: u32 = args.get(0).unwrap().tp.arrtp.unwrap().len();

    let end_block: inkwell::basic_block::BasicBlock = codegen.context.append_basic_block(codegen.enclosing_block.unwrap().get_parent().unwrap(), "end");
    let then_block: inkwell::basic_block::BasicBlock = codegen.context.append_basic_block(codegen.enclosing_block.unwrap().get_parent().unwrap(), "then");
    let else_block: inkwell::basic_block::BasicBlock = codegen.context.append_basic_block(codegen.enclosing_block.unwrap().get_parent().unwrap(), "else");

    let lhs: inkwell::values::IntValue = if std::mem::size_of::<usize>() == std::mem::size_of::<u32>() {
        codegen.inkwell_types.i32tp.const_int(len.into(), false)
    }
    else {
        codegen.inkwell_types.i64tp.const_int(len.into(), false)
    };

    codegen.builder.build_conditional_branch(codegen.builder.build_int_compare(inkwell::IntPredicate::UGT, lhs, args.get(1).unwrap().data.unwrap().into_int_value(), "size_check"), then_block, else_block);

    //
    
    codegen.builder.position_at_end(then_block);
    codegen.enclosing_block = Some(then_block);

    let ptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(args.get(0).unwrap().data.unwrap().into_pointer_value(), 0 as u32, "arr").expect("GEP Error");
    
    let itmptr: inkwell::values::PointerValue = unsafe { codegen.builder.build_in_bounds_gep(ptr, &[codegen.inkwell_types.i32tp.const_zero(), args.get(1).unwrap().data.unwrap().into_int_value()], "itmptr") };

    let itm: inkwell::values::IntValue = codegen.builder.build_load(itmptr, "item").into_int_value();

    let res_some: Data = enums::optionaltype::optional_some(codegen, Some(inkwell::values::BasicValueEnum::IntValue(itm)), opt.types.clone());

    codegen.builder.build_unconditional_branch(end_block);
    
    //

    codegen.builder.position_at_end(else_block);
    codegen.enclosing_block = Some(else_block);

    let res_none: Data = enums::optionaltype::optional_none(codegen, opt.types.clone());
    
    codegen.builder.build_unconditional_branch(end_block);

    //

    let _ = end_block.move_after(else_block);
    codegen.builder.position_at_end(end_block);
    codegen.enclosing_block = Some(end_block);

    codegen.enclosing_block = Some(end_block);

    let mut types: Vec<DataType> = opt.types.clone();
    types.insert(0, crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I32.to_string()).unwrap().clone());

    let phi: inkwell::values::PhiValue = codegen.builder.build_phi(inkwell::types::BasicTypeEnum::StructType(CodeGen::build_struct_tp_from_types(&codegen.context, &codegen.inkwell_types, &types, &codegen.cur_module.datatypes).into_struct_type()), "check_phi");

    phi.add_incoming(&[(&codegen.builder.build_load(res_some.data.unwrap().into_pointer_value(), "some_case"), then_block)]);
    phi.add_incoming(&[(&codegen.builder.build_load(res_none.data.unwrap().into_pointer_value(), "none_case"), else_block)]);

    return Data {
        data: Some(phi.as_basic_value()),
        tp: opt,
        owned: true,
    };
}

pub fn init_array(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    let mut methods: HashMap<String, Method> = HashMap::new();
    
    let tp: DataType = new_datatype(BasicDataType::Array, BasicDataType::Array.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());

    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(array_bool, 1, TraitType::Bool, tp.clone()));
    
    codegen.cur_module.datatypes.insert(BasicDataType::Array.to_string(), tp);

    //length()
    let mut lengthfntp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Func.to_string()).unwrap().clone();
    lengthfntp.name = String::from("length");
    lengthfntp.names = Some(vec![String::from("self")]);
    lengthfntp.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, "usize").unwrap().clone()));
    lengthfntp.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone()];

    methods.insert(String::from("length"), Method {
        tp: MethodType::Builtin,
        builtin: Some(arr_length),
        func: None,
        functp: lengthfntp,
        isinstance: true,
    });
    //

    //length()
    let mut lengthfntp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Func.to_string()).unwrap().clone();
    lengthfntp.name = String::from("get");
    lengthfntp.names = Some(vec![String::from("self")]);
    lengthfntp.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Unknown.to_string()).unwrap().clone()));
    lengthfntp.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone()];

    methods.insert(String::from("get"), Method {
        tp: MethodType::Builtin,
        builtin: Some(array_get),
        func: None,
        functp: lengthfntp,
        isinstance: true,
    });
    //

    let mut alt_tp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone();
    alt_tp.methods = methods;

    codegen.cur_module.datatypes.insert(BasicDataType::Array.to_string(), alt_tp);


    builtin_types::add_simple_type(codegen, traits, BasicDataType::Array, BasicDataType::Array.to_string().as_str());
}