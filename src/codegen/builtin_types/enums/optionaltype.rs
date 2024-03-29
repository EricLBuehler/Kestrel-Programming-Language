use crate::codegen::{self, CodeGen};
use crate::codegen::types::*;

pub fn optional_some<'a>(codegen: &mut codegen::CodeGen<'a>, data: Option<inkwell::values::BasicValueEnum>, types_raw: Vec<DataType<'a>>) -> Data<'a> {
    let optional: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &String::from("Optional")).unwrap().clone();
 
    let mut types: Vec<DataType> = types_raw.clone();
    types.insert(0, crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I32.to_string()).unwrap().clone());

    let st: inkwell::values::PointerValue = CodeGen::alloca(codegen, CodeGen::build_struct_tp_from_types(&codegen.context, &codegen.inkwell_types, &types, &codegen.cur_module.datatypes).into_struct_type(), "enum_st");

    debug_assert_eq!(optional.names.as_ref().unwrap().get(0).unwrap(), &String::from("Some"));
    let id: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 0, "variant_id").expect("GEP Error");
    codegen.builder.build_store(id, codegen.inkwell_types.i32tp.const_int(0, false));
    
    if data.is_some() {
        let variant_data: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 1, "variant_data").expect("GEP Error");
        codegen.builder.build_store(variant_data, data.unwrap());
    }

    return Data {
        data: Some(inkwell::values::BasicValueEnum::PointerValue(st)),
        tp: optional.clone(),
        owned: true
    };
}

pub fn optional_none<'a>(codegen: &mut codegen::CodeGen<'a>, types_raw: Vec<DataType<'a>>) -> Data<'a> {
    let optional: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &String::from("Optional")).unwrap().clone();
 
    let mut types: Vec<DataType> = types_raw.clone();
    types.insert(0, crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::I32.to_string()).unwrap().clone());

    let st: inkwell::values::PointerValue = CodeGen::alloca(codegen, CodeGen::build_struct_tp_from_types(&codegen.context, &codegen.inkwell_types, &types, &codegen.cur_module.datatypes).into_struct_type(), "enum_st");

    debug_assert_eq!(optional.names.as_ref().unwrap().get(1).unwrap(), &String::from("None"));
    let id: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 0, "variant_id").expect("GEP Error");
    codegen.builder.build_store(id, codegen.inkwell_types.i32tp.const_int(0, false));

    return Data {
        data: Some(inkwell::values::BasicValueEnum::PointerValue(st)),
        tp: optional.clone(),
        owned: true
    };
}

pub fn init_optional(codegen: &mut codegen::CodeGen) {
    let mut tp: DataType = crate::codegen::CodeGen::datatypes_get(codegen, &BasicDataType::Enum.to_string()).unwrap().clone();
    tp.name = String::from("Optional");
    tp.names = Some(vec![String::from("Some"), String::from("None")]);
    tp.types = vec![];
    tp.mutability = vec![DataMutablility::Mutable, DataMutablility::Immutable];

    let typ: crate::parser::Type = crate::parser::Type {
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
    
    codegen.cur_module.namespaces.generic_enums.insert(String::from("Optional"), (vec![String::from("T")], vec![Some(typ.to_owned())]));

    codegen.cur_module.datatypes.insert(String::from("Optional"), tp.clone());
    codegen::builtin_types::add_simple_type(codegen, std::collections::HashMap::new(), BasicDataType::Enum, &String::from("Optional"));

}