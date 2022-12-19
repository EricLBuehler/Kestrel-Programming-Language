use crate::codegen::types::{Trait, TraitType, Data, new_datatype, BasicDataType, basic_to_metadata};
use crate::codegen;
use crate::codegen::builtin_types;
use crate::parser;
use std::collections::HashMap;

fn fn_call<'a>(codegen: &codegen::CodeGen<'a>, args: Vec<Data<'a>>, _pos: &parser::Position) -> Data<'a> {
    let args_ref: &Vec<Data> = &args;
    let selfv: &Data = args_ref.first().unwrap();
    let args_: &[Data] = &args_ref[1..];

    let mut args_basic: Vec<inkwell::values::BasicMetadataValueEnum> = Vec::new();

    for arg in args_ {
        let res: Option<inkwell::values::BasicValueEnum> = arg.data;
        if res != None {
            args_basic.push(basic_to_metadata(res.unwrap()));
        }
    }

    let res: inkwell::values::CallSiteValue = codegen.builder.build_call(inkwell::values::CallableValue::try_from(selfv.data.unwrap().into_pointer_value()).unwrap(), &args_basic[..], "res");
    
    if res.try_as_basic_value().is_left() {
        return Data {
            data: Some(res.try_as_basic_value().left().unwrap()),
            tp: selfv.tp.clone(),
        };
    }

    return Data {
        data: None,
        tp: new_datatype(BasicDataType::Unit, BasicDataType::Unit.to_string(),Vec::new(), Vec::new(), Vec::new()),
    };
}

pub fn init_func(codegen: &mut codegen::CodeGen) {
    let mut traits: HashMap<String, Trait> = HashMap::new();
    traits.insert(TraitType::Call.to_string(), builtin_types::create_trait(fn_call, 0, TraitType::Call, BasicDataType::Unknown.to_string(), BasicDataType::Unknown));

    builtin_types::add_simple_type(codegen, traits, BasicDataType::Func, BasicDataType::Func.to_string().as_str());
}