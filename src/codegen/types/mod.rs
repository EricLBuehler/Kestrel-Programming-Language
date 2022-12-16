use crate::codegen;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum DataType {
    I32,
    Unit,
    Func,
}
pub enum TraitType {
    Add,
    Mul,
    Sub,
    Div,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Data<'a> {
    pub data: Option<inkwell::values::BasicValueEnum<'a>>,
    pub tp: DataType,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DataType::I32 => write!(f, "i32"),
            DataType::Unit => write!(f, "Unit"),
            DataType::Func => write!(f, "fn"),
        }
    }    
}

impl std::fmt::Display for TraitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TraitType::Add => write!(f, "Add"),
            TraitType::Mul => write!(f, "Mul"),
            TraitType::Sub => write!(f, "Sub"),
            TraitType::Div => write!(f, "Div"),
        }
    }    
}

pub struct Type<'a> {
    pub attributes: std::collections::HashMap<String, Type<'a>>,
    pub traits: HashMap<String, Trait<'a>>,
    pub methods: std::collections::HashMap<String, Method>,
    pub basictype: DataType,
}

pub struct Trait<'a> {
    pub nargs: usize,
    pub function: fn(&codegen::CodeGen<'a>, Vec<&Data<'a>>, &crate::parser::Position) -> Data<'a>,
    pub traittype: TraitType,
    pub rettp: String,
    pub retbasictype: DataType,
}

pub struct Method {
    pub function: fn(Vec<Type>) -> Type,
}