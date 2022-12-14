use crate::codegen;
use std::collections::HashMap;
use crate::codegen::types::*;
use crate::errors;

fn string_length<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
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
        tp: codegen.datatypes.get("usize").unwrap().clone(),
        owned: true,
    };
}

fn string_get_array<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=1 {
        let fmt: String = format!("Expected 1 argument, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    }
    
    let itmptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(args.get(0).unwrap().data.unwrap().into_pointer_value(), 0 as u32, "arr").expect("GEP Error");
    let arr: inkwell::values::ArrayValue = codegen.builder.build_load(itmptr, "arr").into_array_value();

    let mut tp: DataType = codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone();
    tp.name = codegen::CodeGen::array_repr(arr.get_type());
    tp.arrtp = Some(arr.get_type());
    tp.types = vec![codegen.datatypes.get(&String::from("char")).unwrap().clone()];

    return Data {
        data: Some(inkwell::values::BasicValueEnum::ArrayValue(arr)),
        tp,
        owned: true,
    };
}

fn string_get<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=2 {
        let fmt: String = format!("Expected 2 arguments, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    } 

    if &args.get(1).unwrap().tp != codegen.datatypes.get(&String::from("usize")).unwrap() {
        let fmt: String = format!("invalid types for String.get, expected 'usize', got '{}'.", args.get(1).unwrap().tp);
        errors::raise_error(&fmt, errors::ErrorType::InvalidDataTypes, pos, codegen.info);
    }
    
    let ptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(args.get(0).unwrap().data.unwrap().into_pointer_value(), 0 as u32, "arr").expect("GEP Error");
    
    let itmptr: inkwell::values::PointerValue = unsafe { codegen.builder.build_in_bounds_gep(ptr, &[codegen.inkwell_types.i32tp.const_zero(), args.get(1).unwrap().data.unwrap().into_int_value()], "itmptr") };
    
    let itm: inkwell::values::IntValue = codegen.builder.build_load(itmptr, "item").into_int_value();
    
    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(itm)),
        tp: codegen.datatypes.get(&String::from("char")).unwrap().clone(),
        owned: true,
    };
}

fn string_new<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &crate::parser::Position) -> Data<'a> {
    if args.len()!=1 {
        let fmt: String = format!("Expected 1 argument, got {}.", args.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    }

    let data: &Data = args.get(0).unwrap();

    if data.tp.tp != BasicDataType::Array {
        let fmt: String = format!("Expected array initializer.");
        errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, pos, codegen.info);
    }

    if data.tp.types.first().unwrap() != codegen.datatypes.get(&BasicDataType::I8.to_string()).unwrap() {
        let fmt: String = format!("Expected array of i8.");
        errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, pos, codegen.info);
    }

    let struct_tp: inkwell::types::StructType = codegen.context.struct_type(&[data.data.unwrap().get_type()], false);

    let ptr: inkwell::values::PointerValue = codegen.builder.build_alloca(struct_tp, "String");

    let itmptr: inkwell::values::PointerValue = codegen.builder.build_struct_gep(ptr, 0 as u32, "arr").expect("GEP Error");
    codegen.builder.build_store(itmptr, data.data.unwrap());

    let data: Data = Data {
        data: Some(codegen.builder.build_load(ptr, "string")),
        tp: codegen.datatypes.get(&String::from("String")).unwrap().clone(),
        owned: true,
    };
    return data;
}

pub fn init_string(codegen: &mut codegen::CodeGen) {
    let mut methods: HashMap<String, Method> = HashMap::new();

    let mut tp: DataType = codegen.datatypes.get(&BasicDataType::Struct.to_string()).unwrap().clone();
    tp.name = String::from("String");
    tp.names = Some(vec![String::from("arr")]);
    tp.types = vec![codegen.datatypes.get(&String::from("char")).unwrap().clone()];
    tp.mutability = vec![DataMutablility::Immutable];

    //length()
    let mut lengthfntp: DataType = codegen.datatypes.get(&BasicDataType::Func.to_string()).unwrap().clone();
    lengthfntp.name = String::from("length");
    lengthfntp.names = Some(vec![String::from("self")]);
    lengthfntp.rettp = Some(Box::new(codegen.datatypes.get("usize").unwrap().clone()));
    lengthfntp.types = vec![codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone()];

    methods.insert(String::from("length"), Method {
        tp: MethodType::Builtin,
        builtin: Some(string_length),
        func: None,
        functp: lengthfntp,
        isinstance: true,
    });
    //

    //get_array()
    let mut lengthfntp: DataType = codegen.datatypes.get(&BasicDataType::Func.to_string()).unwrap().clone();
    lengthfntp.name = String::from("get_array");
    lengthfntp.names = Some(vec![String::from("self")]);
    lengthfntp.rettp = Some(Box::new(codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone()));
    lengthfntp.types = vec![codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone()];

    methods.insert(String::from("get_array"), Method {
        tp: MethodType::Builtin,
        builtin: Some(string_get_array),
        func: None,
        functp: lengthfntp,
        isinstance: true,
    });
    //

    //get()
    let mut lengthfntp: DataType = codegen.datatypes.get(&BasicDataType::Func.to_string()).unwrap().clone();
    lengthfntp.name = String::from("get");
    lengthfntp.names = Some(vec![String::from("self"), String::from("index")]);
    lengthfntp.rettp = Some(Box::new(codegen.datatypes.get(&String::from("usize")).unwrap().clone()));
    lengthfntp.types = vec![codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone(), codegen.datatypes.get(&String::from("usize")).unwrap().clone()];

    methods.insert(String::from("get"), Method {
        tp: MethodType::Builtin,
        builtin: Some(string_get),
        func: None,
        functp: lengthfntp,
        isinstance: true,
    });
    //

    //new()
    let mut newfntype: DataType = codegen.datatypes.get(&BasicDataType::Func.to_string()).unwrap().clone();
    newfntype.name = String::from("new");
    newfntype.names = Some(vec![String::from("arr")]);
    newfntype.rettp = Some(Box::new(tp.clone()));
    newfntype.types = vec![codegen.datatypes.get(&BasicDataType::Array.to_string()).unwrap().clone()];

    methods.insert(String::from("new"), Method {
        tp: MethodType::Builtin,
        builtin: Some(string_new),
        func: None,
        functp: newfntype,
        isinstance: false,
    });
    //

    let mut idxmapping: HashMap<String, i32> = HashMap::new();
    idxmapping.insert(String::from("arr"), 0);

    tp.methods = methods;

    codegen.datatypes.insert(String::from("String"), tp.clone());
    codegen.namespaces.structs.insert(String::from("String"), (tp, None, idxmapping, codegen::ForwardDeclarationType::Real));
    codegen::builtin_types::add_simple_type(codegen, std::collections::HashMap::new(), BasicDataType::Struct, &String::from("String"));

}