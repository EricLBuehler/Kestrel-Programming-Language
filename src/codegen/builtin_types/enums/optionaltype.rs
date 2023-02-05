use crate::codegen;
use crate::codegen::types::*;

pub fn init_optional(codegen: &mut codegen::CodeGen) {
    let mut tp: DataType = codegen.datatypes.get(&BasicDataType::Enum.to_string()).unwrap().clone();
    tp.name = String::from("Optional");
    tp.names = Some(vec![String::from("Some"), String::from("None")]);
    tp.types = vec![];
    tp.mutability = vec![DataMutablility::Mutable];

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
    
    codegen.namespaces.generic_enums.insert(String::from("Optional"), (vec![String::from("T")], vec![Some(typ.to_owned()), Some(typ)]));

    codegen.datatypes.insert(String::from("Optional"), tp.clone());
    codegen::builtin_types::add_simple_type(codegen, std::collections::HashMap::new(), BasicDataType::Enum, &String::from("Optional"));

}