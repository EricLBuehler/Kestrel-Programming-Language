use crate::codegen;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum BasicDataType {
    Unknown,
    I32,
    Unit,
    Func,
    U32,
    I8,
    U8,
    I16,
    U16,
    I64,
    U64,
    I128,
    U128,
}
#[derive(Clone, Debug)]
pub struct DataType {
    pub tp: BasicDataType,
    pub names: Option<Vec<String>>,
    pub types: Vec<DataType>,
    pub name: String,
    pub mutability: Vec<DataMutablility>,
    pub rettp: Vec<DataType>, //Just for indirection
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
            BasicDataType::U32 => write!(f, "u32"),
            BasicDataType::I8 => write!(f, "i8"),
            BasicDataType::U8 => write!(f, "u8"),
            BasicDataType::I16 => write!(f, "i16"),
            BasicDataType::U16 => write!(f, "u16"),
            BasicDataType::I64 => write!(f, "i64"),
            BasicDataType::U64 => write!(f, "u64"),
            BasicDataType::I128 => write!(f, "i128"),
            BasicDataType::U128 => write!(f, "u128"),
        }
    }    
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tp)
    }    
}

impl PartialEq for DataType {
    fn eq(&self, other: &DataType) -> bool {
        return self.name == other.name;
    }
    fn ne(&self, other: &DataType) -> bool {
        return self.name != other.name;
    }
}

impl PartialEq<BasicDataType> for DataType {
    fn eq(&self, other: &BasicDataType) -> bool {
        return self.name == other.to_string();
    }
    fn ne(&self, other: &BasicDataType) -> bool {
        return self.name != other.to_string();
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
    pub rettp: DataType
}

pub struct Method {
    pub function: fn(Vec<Type>) -> Type,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataMutablility{
    Immutable,
    Mutable,
}

pub fn new_datatype(tp: BasicDataType, name: String, names: Option<Vec<String>>, types: Vec<DataType>, mutability: Vec<DataMutablility>, rettp_opt: Option<DataType>) -> DataType {
    return DataType {
        tp,
        names,
        types,
        name,
        mutability,
        rettp: if rettp_opt.is_some() {vec![rettp_opt.unwrap()]} else {Vec::new()}
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