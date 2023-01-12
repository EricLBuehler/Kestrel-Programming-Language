use crate::codegen::types::{Trait, TraitType, Data, DataType, new_datatype, BasicDataType, basic_to_metadata};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use crate::errors;
use std::collections::HashMap;

pub fn fn_call<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, pos: &parser::Position) -> Data<'a> {
    let args_ref: &Vec<Data> = &args;
    let selfv: &Data = args_ref.first().unwrap();
    let args_: &[Data] = &args_ref[1..];

    let mut args_basic: Vec<inkwell::values::BasicMetadataValueEnum> = Vec::new();
    let types: &Vec<DataType> = &selfv.tp.types;
    
    if args_.len() != selfv.tp.names.as_ref().unwrap().len(){
        let fmt: String = format!("Expected {} arguments, got {}.", selfv.tp.names.as_ref().unwrap().len(), args_.len());
        errors::raise_error(&fmt, errors::ErrorType::ArgumentCountMismatch, pos, codegen.info);
    }

    let mut idx: usize = 0;
    for arg in args_ {
        let res: Option<inkwell::values::BasicValueEnum> = arg.data;
        if res != None {
            args_basic.push(basic_to_metadata(res.unwrap()));
        }
        if arg.tp != *types.get(idx).unwrap(){
            let fmt: String = format!("expected '{}' type, got '{}' type.", types.get(idx).unwrap().name, arg.tp.name);
            errors::raise_error(&fmt, errors::ErrorType::TypeMismatch, pos, codegen.info);
        }
        idx += 1;
    }

    let res: inkwell::values::CallSiteValue = codegen.builder.build_call(inkwell::values::CallableValue::try_from(selfv.data.unwrap().into_pointer_value()).unwrap(), &args_basic[..], "res");
    
    if res.try_as_basic_value().is_left() {
        return Data {
            data: Some(res.try_as_basic_value().left().unwrap()),
            tp: *selfv.tp.rettp.as_ref().unwrap().clone(),
            owned: true,
        };
    }

    return Data {
        data: None,
        tp: codegen.datatypes.get(&BasicDataType::Void.to_string()).unwrap().clone(),
        owned: true,
    };
}

fn func_bool<'a>(codegen: &codegen::CodeGen<'a>, _args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    return Data {
        data: Some(inkwell::values::BasicValueEnum::IntValue(codegen.inkwell_types.booltp.const_int(0, false))),
        tp: codegen.datatypes.get(&BasicDataType::Bool.to_string()).unwrap().clone(),
        owned: true,
    };
}

pub fn init_func(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Call.to_string(), builtin_types::create_trait_func(fn_call, 0, TraitType::Call, codegen.datatypes.get(&BasicDataType::Unknown.to_string()).unwrap().clone()));

    let tp: DataType = new_datatype(BasicDataType::Func, BasicDataType::Func.to_string(), None, Vec::new(), Vec::new(), None, false, None, std::collections::HashMap::new());
    
    traits.insert(TraitType::Bool.to_string(), builtin_types::create_trait_func(func_bool, 1, TraitType::Bool, tp.clone()));

    codegen.datatypes.insert(BasicDataType::Func.to_string(), tp.clone());

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Func, BasicDataType::Func.to_string().as_str());
}