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
    F32,
    F64,
    Struct,
    Array,
}
#[derive(Clone, Debug)]
pub struct DataType<'a> {
    pub tp: BasicDataType,
    pub names: Option<Vec<String>>,
    pub types: Vec<DataType<'a>>,
    pub name: String,
    pub mutability: Vec<DataMutablility>,
    pub rettp: Vec<DataType<'a>>, //Just for indirection
    pub is_ref: bool,
    pub arrtp: Option<inkwell::types::ArrayType<'a>>,
}
pub enum TraitType {
    Add,
    Mul,
    Sub,
    Div,
    Call,
    Neg,
    Pos,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Data<'a> {
    pub data: Option<inkwell::values::BasicValueEnum<'a>>,
    pub tp: DataType<'a>,
    pub owned: bool,
}

impl std::fmt::Display for BasicDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BasicDataType::I32 => write!(f, "i32"),
            BasicDataType::Unit => write!(f, "void"),
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
            BasicDataType::F32 => write!(f, "f32"),
            BasicDataType::F64 => write!(f, "f64"),
            BasicDataType::Struct => write!(f, "Struct"),
            BasicDataType::Array => write!(f, "Array"),
        }
    }    
}

impl<'a> std::fmt::Display for DataType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }    
}

impl<'a> PartialEq for DataType<'a> {
    fn eq(&self, other: &DataType) -> bool {
        return self.name == other.name;
    }
    fn ne(&self, other: &DataType) -> bool {
        return self.name != other.name;
    }
}

impl<'a> PartialEq<BasicDataType> for DataType<'a> {
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
            TraitType::Pos => write!(f, "Pos"),
            TraitType::Neg => write!(f, "Neg"),
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
    pub rettp: DataType<'a>
}

pub struct Method {
    pub function: fn(Vec<Type>) -> Type,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataMutablility{
    Immutable,
    Mutable,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DataOwnership{
    pub owned: bool,
    pub transferred: Option<crate::parser::Position>,
}

pub fn new_datatype<'a>(tp: BasicDataType, name: String, names: Option<Vec<String>>, types: Vec<DataType<'a>>, mutability: Vec<DataMutablility>, rettp_opt: Option<DataType<'a>>, is_ref: bool, arrtp: Option<inkwell::types::ArrayType<'a>>) -> DataType<'a> {
    return DataType {
        tp,
        names,
        types,
        name,
        mutability,
        rettp: if rettp_opt.is_some() {vec![rettp_opt.unwrap()]} else {Vec::new()},
        is_ref,
        arrtp,
    };
}

pub fn basic_to_metadata(basic: inkwell::values::BasicValueEnum) -> inkwell::values::BasicMetadataValueEnum{
    if basic.is_int_value() {
        return inkwell::values::BasicMetadataValueEnum::IntValue(basic.into_int_value());
    }
    else if basic.is_pointer_value() {
        return inkwell::values::BasicMetadataValueEnum::PointerValue(basic.into_pointer_value());
    }
    else if basic.is_float_value() {
        return inkwell::values::BasicMetadataValueEnum::FloatValue(basic.into_float_value());
    }

    unimplemented!("basic_to_metadata");
}