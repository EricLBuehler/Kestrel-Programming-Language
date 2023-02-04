use crate::codegen;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum BasicDataType {
    Unknown,
    I32,
    Void,
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
    WrapperFunc,
    Bool,
    Enum,
    Dyn,
}

#[derive(Clone)]
pub struct DataType<'a> {
    pub tp: BasicDataType,
    pub names: Option<Vec<String>>,
    pub types: Vec<DataType<'a>>,
    pub name: String,
    pub mutability: Vec<DataMutablility>,
    pub rettp: Option<Box<DataType<'a>>>,
    pub is_ref: bool,
    pub is_dyn: bool,
    pub arrtp: Option<inkwell::types::ArrayType<'a>>,
    pub wrapperfn: Option<fn(&codegen::CodeGen<'a>, Vec<Data<'a>>, &crate::parser::Position) -> Data<'a>>,
    pub methods: std::collections::HashMap<String, Method<'a>>,
    pub lifetime: Option<DataLifetime>,
}

impl<'a> std::fmt::Debug for DataType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tp)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TraitType {
    Add,
    Mul,
    Sub,
    Div,
    Call,
    Neg,
    Pos,
    Bool,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    User,
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
            BasicDataType::Void => write!(f, "void"),
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
            BasicDataType::WrapperFunc => write!(f, "WrapperFn"),
            BasicDataType::Bool => write!(f, "bool"),
            BasicDataType::Enum => write!(f, "enum"),
            BasicDataType::Dyn => write!(f, "dyn"),
        }
    }    
}

impl<'a> std::fmt::Display for DataType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix: String;
        if self.is_dyn {
            prefix = String::from("dyn ");
        }
        else {
            prefix = String::from("");
        }
        write!(f, "{}{}", prefix, self.name)
    }    
}

impl<'a> PartialEq for DataType<'a> {
    fn eq(&self, other: &DataType<'a>) -> bool {
        if self.arrtp.is_some() && other.arrtp.is_some() {
            return self.arrtp.unwrap() == other.arrtp.unwrap();
        }

        if self.is_dyn {
            if self.name == other.name && other.is_dyn {
                return true;
            }
            return false;
        }

        if self.tp==BasicDataType::Func && other.tp==BasicDataType::Func {
            if  self.types != other.types || 
                self.rettp != other.rettp ||
                self.mutability != other.mutability {
                    return false;
                }
                
            if self.names.is_some() && other.names.is_some() {
                if self.names.as_ref().unwrap() != other.names.as_ref().unwrap() {
                    return false
                }
            }
            
            return true;
        }

        if self.tp==BasicDataType::Struct && other.tp==BasicDataType::Struct {
            if  self.types != other.types ||
                self.names.as_ref().unwrap() != other.names.as_ref().unwrap() ||
                self.name != other.name {
                return false;
            }
        }

        if self.tp==BasicDataType::Enum && other.tp==BasicDataType::Enum {
            if  self.types != other.types ||
                self.name != other.name {
                return false;
            }
        }

        if self.is_ref == other.is_ref {
            if  self.name != other.name &&
                self.mutability != other.mutability {
                return false;
            }
        }

        return self.name == other.name;
    }
    fn ne(&self, other: &DataType<'a>) -> bool {
        if self.arrtp.is_some() && other.arrtp.is_some() {
            return self.arrtp.unwrap() != other.arrtp.unwrap();
        }

        if self.is_dyn {
            if self.name != other.name || !other.is_dyn {
                return true;
            }
            return false;
        }

        if self.tp==BasicDataType::Func && other.tp==BasicDataType::Func {
            if  self.types == other.types && 
                self.rettp == other.rettp &&
                self.mutability == other.mutability {
                    if self.names.is_some() && other.names.is_some() {
                        if self.names.as_ref().unwrap() == other.names.as_ref().unwrap() {
                            return true;
                        }
                    }
                }
                
            return false;
        }

        if self.tp==BasicDataType::Struct && other.tp==BasicDataType::Struct {
            if  self.types == other.types &&
                self.names.as_ref().unwrap() == other.names.as_ref().unwrap() &&
                self.name == other.name {
                return false;
            }
        }

        if self.tp==BasicDataType::Enum && other.tp==BasicDataType::Enum {
            if  self.types == other.types &&
                self.name == other.name {
                return false;
            }
        }

        if self.is_ref == other.is_ref {
            if  self.name == other.name &&
                self.mutability == other.mutability {
                return false;
            }
        }

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
            TraitType::Bool => write!(f, "Bool"),
            TraitType::Eq => write!(f, "Eq"),
            TraitType::Lt => write!(f, "Lt"),
            TraitType::Gt => write!(f, "Gt"),
            TraitType::Le => write!(f, "Le"),
            TraitType::Ge => write!(f, "Ge"),
            TraitType::Ne => write!(f, "Ne"),
            TraitType::User => write!(f, "User"),
        }
    }    
}

#[derive(Clone)]
pub struct Type<'a> {
    pub traits: HashMap<String, Trait<'a>>,
    pub basictype: BasicDataType,
}

#[derive(Clone)]
pub struct Trait<'a> {
    pub nargs: usize,
    pub function: Option<fn(&codegen::CodeGen<'a>, Vec<Data<'a>>, &crate::parser::Position) -> Data<'a>>,
    pub inkfunc: Option<inkwell::values::PointerValue<'a>>,
    pub traittype: TraitType,
    pub rettp: Option<DataType<'a>>,
}

impl<'a> std::fmt::Display for Trait<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.traittype)
    }
}

impl<'a> std::fmt::Debug for Trait<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.traittype)
    }
}

#[derive(Clone)]
pub struct TraitSignature<'a> {
    pub traittp: TraitMetatype,
    pub name: String,
    pub nargs: Option<usize>,
    pub trait_sig: Option<Vec<TemplateTraitSignature>>,
    pub vars: Option<std::collections::HashMap<String, crate::parser::Type>>,
    pub implementations: std::collections::HashMap<String, std::collections::HashMap<String, inkwell::values::PointerValue<'a>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateTraitSignature {
    pub name: String,
    pub methodname: Option<String>,
    pub namespacename: Option<String>,
    pub template_types: Vec<String>,
    pub args: crate::parser::Args,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TraitMetatype {
    Builtin,
    User,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataMutablility{
    Immutable,
    Mutable,
}

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum DataLifetime{
    Local,
    Heap(String),
    None,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DataOwnership{
    pub owned: bool,
    pub transferred: Option<crate::parser::Position>,
    pub mut_borrowed: bool,
}

pub fn new_datatype<'a>(tp: BasicDataType, name: String, names: Option<Vec<String>>, types: Vec<DataType<'a>>, mutability: Vec<DataMutablility>, rettp_opt: Option<DataType<'a>>, is_ref: bool, arrtp: Option<inkwell::types::ArrayType<'a>>, methods: std::collections::HashMap<String, Method<'a>>) -> DataType<'a> {
    return DataType {
        tp,
        names,
        types,
        name,
        mutability,
        rettp: if rettp_opt.is_some() {Some(Box::new(rettp_opt.unwrap()))} else {None},
        is_ref,
        is_dyn: false,
        arrtp,
        wrapperfn: None,
        methods,
        lifetime: None,
    };
}

pub fn new_dyn_datatype<'a>(traitnm: String, mutability: DataMutablility) -> DataType<'a> {
    return DataType {
        tp: BasicDataType::Dyn,
        names: None,
        types: Vec::new(),
        name: traitnm,
        mutability: vec![mutability],
        rettp: None,
        is_ref: false,
        is_dyn: true,
        arrtp: None,
        wrapperfn: None,
        methods: std::collections::HashMap::new(),
        lifetime: None,
    };
}

pub fn basic_to_metadata(basic: inkwell::values::BasicValueEnum) -> inkwell::values::BasicMetadataValueEnum{
    if basic.is_int_value() {
        return inkwell::values::BasicMetadataValueEnum::IntValue(basic.into_int_value());
    }
    else if basic.is_pointer_value() {
        return inkwell::values::BasicMetadataValueEnum::PointerValue(basic.into_pointer_value());
    }
    else if basic.is_struct_value() {
        return inkwell::values::BasicMetadataValueEnum::StructValue(basic.into_struct_value());
    }
    else if basic.is_float_value() {
        return inkwell::values::BasicMetadataValueEnum::FloatValue(basic.into_float_value());
    }

    unimplemented!("basic_to_metadata");
}

#[derive(Clone, Debug, PartialEq)]
pub enum MethodType {
    Builtin,
    Fn,
}

#[derive(Clone)]
pub struct Method<'a> {
    pub tp: MethodType,
    pub builtin: Option<fn(&codegen::CodeGen<'a>, Vec<Data<'a>>, &crate::parser::Position) -> Data<'a>>,
    pub func: Option<inkwell::values::PointerValue<'a>>,
    pub functp: DataType<'a>,
    pub isinstance: bool,
}

impl<'a> std::fmt::Debug for Method<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tp)
    }
}

pub fn get_traittp_from_str(tp: String) -> Option<TraitType> {
    if tp == TraitType::Add.to_string() {
        return Some(TraitType::Add);
    }
    else if tp == TraitType::Sub.to_string() {
        return Some(TraitType::Sub);
    }
    else if tp == TraitType::Mul.to_string() {
        return Some(TraitType::Mul);
    }
    else if tp == TraitType::Div.to_string() {
        return Some(TraitType::Div);
    }
    else if tp == TraitType::Pos.to_string() {
        return Some(TraitType::Pos);
    }
    else if tp == TraitType::Neg.to_string() {
        return Some(TraitType::Neg);
    }
    else if tp == TraitType::Bool.to_string() {
        return Some(TraitType::Bool);
    }
    else if tp == TraitType::Call.to_string() {
        return Some(TraitType::Call);
    }
    
    return None;
}