use crate::codegen;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum BasicDataType {
    Unknown,
    I32,
    Unit,
    Func,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DataType {
    pub tp: BasicDataType,
    pub names: Vec<String>,
    pub types: Vec<DataType>,
    pub name: String,
    pub mutability: Vec<DataMutablility>,
}
pub enum TraitType {
    Add,
    Mul,
    Sub,
    Div,
    Call,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Data<'a> {
    pub data: Option<inkwell::values::BasicValueEnum<'a>>,
    pub tp: DataType,
}

impl std::fmt::Display for BasicDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BasicDataType::I32 => write!(f, "i32"),
            BasicDataType::Unit => write!(f, "unit"),
            BasicDataType::Func => write!(f, "fn"),
            BasicDataType::Unknown => write!(f, "UNKNOWN"),
        }
    }    
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tp)
    }    
}

impl PartialEq<BasicDataType> for DataType {
    fn eq(&self, other: &BasicDataType) -> bool {
        return self.tp == *other;
    }
    fn ne(&self, other: &BasicDataType) -> bool {
        return self.tp != *other;
    }
}

impl std::fmt::Display for TraitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TraitType::Add => write!(f, "Add"),
            TraitType::Mul => write!(f, "Mul"),
            TraitType::Sub => write!(f, "Sub"),
            TraitType::Div => write!(f, "Div"),
            TraitType::Call => write!(f, "Call"),
        }
    }    
}

pub struct Type<'a> {
    pub attributes: std::collections::HashMap<String, Type<'a>>,
    pub traits: HashMap<String, Trait<'a>>,
    pub methods: std::collections::HashMap<String, Method>,
    pub basictype: BasicDataType,
}

pub struct Trait<'a> {
    pub nargs: usize,
    pub function: fn(&codegen::CodeGen<'a>, Vec<Data<'a>>, &crate::parser::Position) -> Data<'a>,
    pub traittype: TraitType,
    pub rettp: String,
    pub retbasictype: BasicDataType,
}

pub struct Method {
    pub function: fn(Vec<Type>) -> Type,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataMutablility{
    Immutable,
    Mutable,
}

pub fn new_datatype(tp: BasicDataType, name: String, names: Vec<String>, types: Vec<DataType>, mutability: Vec<DataMutablility>) -> DataType {
    return DataType {
        tp,
        names,
        types,
        name,
        mutability,
    };
}

pub fn basic_to_metadata(basic: inkwell::values::BasicValueEnum) -> inkwell::values::BasicMetadataValueEnum{
    if basic.is_int_value() {
        return inkwell::values::BasicMetadataValueEnum::IntValue(basic.into_int_value());
    }
    else if basic.is_pointer_value() {
        return inkwell::values::BasicMetadataValueEnum::PointerValue(basic.into_pointer_value());
    }

    unimplemented!("basic_to_metadata");
}