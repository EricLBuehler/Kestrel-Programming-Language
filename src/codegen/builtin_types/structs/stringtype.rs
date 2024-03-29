use crate::codegen::{self, CodeGen};
use crate::codegen::builtin_types::enums;
use std::collections::HashMap;
use crate::codegen::types::*;
use crate::errors;

fn string_length<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=1 {
        let fmt: String = format!("Expected 1 argument, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    }
    
    let itmptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(args.get(0).unwrap().data.unwrap().into_pointer_value(), 0 as u32, "arr").expect("GEP Error");
    let arr: inkwell::values::ArrayValue = codegen.builder.build_load(itmptr, "arr").into_array_value();

    let len: u32 = arr.get_type().len();

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

fn string_get_array<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=1 {
        let fmt: String = format!("Expected 1 argument, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    }
    
    let itmptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(args.get(0).unwrap().data.unwrap().into_pointer_value(), 0 as u32, "arr").expect("GEP Error");
    let arr: inkwell::values::ArrayValue = codegen.builder.build_load(itmptr, "arr").into_array_value();

    let mut tp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone();
    tp.name = codegen::CodeGen::array_repr(arr.get_type());
    tp.arrtp = Some(arr.get_type());
    tp.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &crate::codegen::types::BasicDataType::U8.to_string()).unwrap().clone()];

    return Data {
        data: Some(inkwell::values::BasicValueEnum::ArrayValue(arr)),
        tp,
        owned: false,
    };
}

fn string_get<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=2 {
        let fmt: String = format!("Expected 2 arguments, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    } 

    if args.get(1).unwrap().tp != crate::codegen::CodeGen::datatypes_get(codegen, &String::from("usize")).unwrap() {
        let fmt: String = format!("Invalid types for String.get, expected 'usize', got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }

    let mut opt: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &String::from("Optional")).unwrap().clone();
    opt.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::U8.to_string()).unwrap().clone(), crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I32.to_string()).unwrap().clone()];

    let ptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(args.get(0).unwrap().data.unwrap().into_pointer_value(), 0 as u32, "arr").expect("GEP Error");
    let arr: inkwell::values::ArrayValue = codegen.builder.build_load(ptr, "load_arr").into_array_value();

    let len: u32 = arr.get_type().len();

    let end_block: inkwell::basic_block::BasicBlock = codegen.context.append_basic_block(codegen.current_block.unwrap().get_parent().unwrap(), "end");
    let then_block: inkwell::basic_block::BasicBlock = codegen.context.append_basic_block(codegen.current_block.unwrap().get_parent().unwrap(), "then");
    let else_block: inkwell::basic_block::BasicBlock = codegen.context.append_basic_block(codegen.current_block.unwrap().get_parent().unwrap(), "else");

    let lhs: inkwell::values::IntValue = if std::mem::size_of::<usize>() == std::mem::size_of::<u32>() {
        codegen.inkwell_types.i32tp.const_int(len.into(), false)
    }
    else {
        codegen.inkwell_types.i64tp.const_int(len.into(), false)
    };

    codegen.builder.build_conditional_branch(codegen.builder.build_int_compare(inkwell::IntPredicate::UGT, lhs, args.get(1).unwrap().data.unwrap().into_int_value(), "size_check"), then_block, else_block);

    //
    
    codegen.builder.position_at_end(then_block);
    codegen.current_block = Some(then_block);

    let itmptr: inkwell::values::PointerValue = unsafe { codegen.builder.build_in_bounds_gep(ptr, &[codegen.inkwell_types.i32tp.const_zero(), args.get(1).unwrap().data.unwrap().into_int_value()], "itmptr") };
    
    let itm: inkwell::values::IntValue = codegen.builder.build_load(itmptr, "item").into_int_value();

    let res_some: Data = enums::optionaltype::optional_some(codegen, Some(inkwell::values::BasicValueEnum::IntValue(itm)), opt.types.clone());

    codegen.builder.build_unconditional_branch(end_block);
    
    //

    codegen.builder.position_at_end(else_block);
    codegen.current_block = Some(else_block);

    let res_none: Data = enums::optionaltype::optional_none(codegen, opt.types.clone());
    
    codegen.builder.build_unconditional_branch(end_block);

    //

    let _ = end_block.move_after(else_block);
    codegen.builder.position_at_end(end_block);
    codegen.current_block = Some(end_block);

    codegen.current_block = Some(end_block);


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

fn string_new<'a>(codegen: &mut codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=1 {
        let fmt: String = format!("Expected 1 argument, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    }

    let data: &Data = args.get(0).unwrap();

    if data.tp.tp != BasicDataType::Array {
        let fmt: String = format!("Expected array initializer.");
        errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, pos, codegen.info);
    }

    if *data.tp.types.first().unwrap() != crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I8.to_string()).unwrap() {
        let fmt: String = format!("Expected array of i8.");
        errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, pos, codegen.info);
    }

    let struct_tp: inkwell::types::StructType = codegen.context.struct_type(&[data.data.unwrap().get_type()], false);

    let ptr: inkwell::values::PointerValue = CodeGen::alloca(codegen, struct_tp, "String");

    let itmptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(ptr, 0 as u32, "arr").expect("GEP Error");
    codegen.builder.build_store(itmptr, data.data.unwrap());
    
    let data: Data = Data {
        data: Some(codegen.builder.build_load(ptr, "string")),
        tp: crate::codegen::CodeGen::datatypes_get(codegen, &String::from("String")).unwrap().clone(),
        owned: true,
    };
    return data;
}

pub fn init_string(codegen: &mut codegen::CodeGen) {
    let mut methods: HashMap<String, Method> = HashMap::new();

    let mut tp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Struct.to_string()).unwrap().clone();
    tp.name = String::from("String");
    tp.names = Some(vec![String::from("arr")]);
    tp.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &String::from("char")).unwrap().clone()];
    tp.mutability = vec![DataMutablility::Immutable];

    //length()
    let mut lengthfntp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::WrapperFunc.to_string()).unwrap().clone();
    lengthfntp.names = Some(vec![String::from("self")]);
    lengthfntp.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, "usize").unwrap().clone()));
    lengthfntp.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone()];
    lengthfntp.wrapperfn = Some(string_length);

    methods.insert(String::from("length"), Method {
        tp: MethodType::Builtin,
        builtin: Some(string_length),
        func: None,
        functp: lengthfntp,
        isinstance: true,
        isinstanceptr: false,
        ismutinstanceptr: false,
    });
    //

    //get_array()
    let mut getarrfntp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::WrapperFunc.to_string()).unwrap().clone();
    getarrfntp.names = Some(vec![String::from("self")]);
    getarrfntp.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone()));
    getarrfntp.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone()];
    getarrfntp.wrapperfn = Some(string_get_array);

    methods.insert(String::from("get_array"), Method {
        tp: MethodType::Builtin,
        builtin: Some(string_get_array),
        func: None,
        functp: getarrfntp,
        isinstance: true,
        isinstanceptr: false,
        ismutinstanceptr: false,
    });
    //

    //get()
    let mut getfntp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::WrapperFunc.to_string()).unwrap().clone();
    getfntp.names = Some(vec![String::from("self"), String::from("index")]);
    getfntp.rettp = Some(Box::new(crate::codegen::CodeGen::datatypes_get(codegen, &crate::codegen::types::BasicDataType::U8.to_string()).unwrap().clone()));
    getfntp.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone(), crate::codegen::CodeGen::datatypes_get(codegen, &String::from("usize")).unwrap().clone()];
    getfntp.wrapperfn = Some(string_get);

    methods.insert(String::from("get"), Method {
        tp: MethodType::Builtin,
        builtin: Some(string_get),
        func: None,
        functp: getfntp,
        isinstance: true,
        isinstanceptr: false,
        ismutinstanceptr: false,
    });
    //

    //new()
    let mut newfntype: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::WrapperFunc.to_string()).unwrap().clone();
    newfntype.name = String::from("new");
    newfntype.names = Some(vec![String::from("arr")]);
    newfntype.rettp = Some(Box::new(tp.clone()));
    newfntype.types = vec![crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Array.to_string()).unwrap().clone()];
    newfntype.wrapperfn = Some(string_new);

    methods.insert(String::from("new"), Method {
        tp: MethodType::Builtin,
        builtin: Some(string_new),
        func: None,
        functp: newfntype,
        isinstance: false,
        isinstanceptr: false,
        ismutinstanceptr: false,
    });
    //

    let mut idxmapping: HashMap<String, i32> = HashMap::new();
    idxmapping.insert(String::from("arr"), 0);

    tp.methods = methods;

    codegen.cur_module.datatypes.insert(String::from("String"), tp.clone());
    codegen.cur_module.namespaces.structs.insert(String::from("String"), (tp, None, idxmapping, codegen::ForwardDeclarationType::Real));
    codegen::builtin_types::add_simple_type(codegen, std::collections::HashMap::new(), BasicDataType::Struct, &String::from("String"));

}