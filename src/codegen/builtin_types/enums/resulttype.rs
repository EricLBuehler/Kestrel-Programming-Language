use crate::codegen::{self, CodeGen};
use crate::codegen::types::*;

#[allow(dead_code)]
pub fn result_ok<'a>(codegen: &codegen::CodeGen<'a>, data: Option<inkwell::values::BasicValueEnum>, types_raw: Vec<DataType<'a>>) -> Data<'a> {
    let result: DataType = codegen.datatypes.get(&String::from("Result")).unwrap().clone();
 
    let mut types: Vec<DataType> = types_raw.clone();
    types.insert(0, codegen.datatypes.get(&BasicDataType::I32.to_string()).unwrap().clone());

    let st: inkwell::values::PointerValue = codegen.builder.build_alloca(CodeGen::build_struct_tp_from_types(&codegen.context, &codegen.inkwell_types, &types, &codegen.datatypes).into_struct_type(), "enum_st");

    debug_assert_eq!(result.names.as_ref().unwrap().get(0).unwrap(), &String::from("Ok"));
    let id: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 0, "variant_id").expect("GEP Error");
    codegen.builder.build_store(id, codegen.inkwell_types.i32tp.const_int(0, false));
    
    if data.is_some() {
        let variant_data: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 1, "variant_data").expect("GEP Error");
        codegen.builder.build_store(variant_data, data.unwrap());
    }

    return Data {
        data: Some(inkwell::values::BasicValueEnum::PointerValue(st)),
        tp: result.clone(),
        owned: true
    };
}

#[allow(dead_code)]
pub fn result_err<'a>(codegen: &codegen::CodeGen<'a>, data: Option<inkwell::values::BasicValueEnum>, types_raw: Vec<DataType<'a>>) -> Data<'a> {
    let result: DataType = codegen.datatypes.get(&String::from("Result")).unwrap().clone();
 
    let mut types: Vec<DataType> = types_raw.clone();
    types.insert(0, codegen.datatypes.get(&BasicDataType::I32.to_string()).unwrap().clone());

    let st: inkwell::values::PointerValue = codegen.builder.build_alloca(CodeGen::build_struct_tp_from_types(&codegen.context, &codegen.inkwell_types, &types, &codegen.datatypes).into_struct_type(), "enum_st");

    debug_assert_eq!(result.names.as_ref().unwrap().get(1).unwrap(), &String::from("Err"));
    let id: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 0, "variant_id").expect("GEP Error");
    codegen.builder.build_store(id, codegen.inkwell_types.i32tp.const_int(0, false));
    
    if data.is_some() {
        let variant_data: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 2, "variant_data").expect("GEP Error");
        codegen.builder.build_store(variant_data, data.unwrap());
    }

    return Data {
        data: Some(inkwell::values::BasicValueEnum::PointerValue(st)),
        tp: result.clone(),
        owned: true
    };
}

pub fn init_result(codegen: &mut codegen::CodeGen) {
    let mut tp: DataType = codegen.datatypes.get(&BasicDataType::Enum.to_string()).unwrap().clone();
    tp.name = String::from("Result");
    tp.names = Some(vec![String::from("Ok"), String::from("Err")]);
    tp.types = vec![];
    tp.mutability = vec![DataMutablility::Mutable, DataMutablility::Immutable];

    let typ_t: crate::parser::Type = crate::parser::Type {
        isfn: false,
        isarr: false,
        isdyn: false,
        isgenum: false,
        isref: false,
        basetp: None,
        arrlen: None,
        data: Some(String::from("T")),
        args: None,
        mutability: DataMutablility::Immutable,
        refmutability: None,
        generic_tps: None,
    };

    let typ_e: crate::parser::Type = crate::parser::Type {
        isfn: false,
        isarr: false,
        isdyn: false,
        isgenum: false,
        isref: false,
        basetp: None,
        arrlen: None,
        data: Some(String::from("E")),
        args: None,
        mutability: DataMutablility::Immutable,
        refmutability: None,
        generic_tps: None,
    };
    
    codegen.namespaces.generic_enums.insert(String::from("Result"), (vec![String::from("T"), String::from("E")], vec![Some(typ_t), Some(typ_e)]));

    codegen.datatypes.insert(String::from("Result"), tp.clone());
    codegen::builtin_types::add_simple_type(codegen, std::collections::HashMap::new(), BasicDataType::Enum, &String::from("Result"));

}