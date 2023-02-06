use crate::codegen;
use crate::codegen::types::*;

#[allow(dead_code)]
pub fn result_ok<'a>(codegen: &codegen::CodeGen<'a>, data: Option<inkwell::values::BasicValueEnum>) -> Data<'a> {
    let st: inkwell::values::PointerValue = codegen.builder.build_alloca(*codegen.inkwell_types.enumsttp, "enum_st");

    let id: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 0, "variant_id").expect("GEP Error");
    let result: DataType = codegen.datatypes.get(&String::from("Result")).unwrap().clone();

    debug_assert_eq!(result.names.as_ref().unwrap().get(0).unwrap(), &String::from("Ok"));
    codegen.builder.build_store(id, codegen.inkwell_types.i32tp.const_int(0, false));
    
    if data.is_some() {
        let data_ptr: inkwell::values::PointerValue = codegen.builder.build_alloca(data.unwrap().get_type(), "variant_data_ptr");
        codegen.builder.build_store(data_ptr, data.unwrap());

        let data_bitcast: inkwell::values::PointerValue = codegen.builder.build_bitcast(data_ptr, codegen.inkwell_types.enum_data_tp.ptr_type(inkwell::AddressSpace::from(0u16)), "variant_data_bitcast").into_pointer_value();
        let variant_data: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 1, "variant_data").expect("GEP Error");
        codegen.builder.build_store(variant_data, data_bitcast);
    }

    return Data {
        data: Some(inkwell::values::BasicValueEnum::PointerValue(st)),
        tp: result.clone(),
        owned: true
    };
}

#[allow(dead_code)]
pub fn result_err<'a>(codegen: &codegen::CodeGen<'a>, data: Option<inkwell::values::BasicValueEnum>) -> Data<'a> {
    let st: inkwell::values::PointerValue = codegen.builder.build_alloca(*codegen.inkwell_types.enumsttp, "enum_st");

    let id: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 0, "variant_id").expect("GEP Error");
    let result: DataType = codegen.datatypes.get(&String::from("Result")).unwrap().clone();

    debug_assert_eq!(result.names.as_ref().unwrap().get(1).unwrap(), &String::from("Err"));
    codegen.builder.build_store(id, codegen.inkwell_types.i32tp.const_int(1, false));
    
    if data.is_some() {
        let data_ptr: inkwell::values::PointerValue = codegen.builder.build_alloca(data.unwrap().get_type(), "variant_data_ptr");
        codegen.builder.build_store(data_ptr, data.unwrap());

        let data_bitcast: inkwell::values::PointerValue = codegen.builder.build_bitcast(data_ptr, codegen.inkwell_types.enum_data_tp.ptr_type(inkwell::AddressSpace::from(0u16)), "variant_data_bitcast").into_pointer_value();
        let variant_data: inkwell::values::PointerValue = codegen.builder.build_struct_gep(st, 1, "variant_data").expect("GEP Error");
        codegen.builder.build_store(variant_data, data_bitcast);
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