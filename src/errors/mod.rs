use colored::Colorize;

#[derive(Clone)]
pub enum ErrorType {
    InvalidTok,    
    InvalidDataTypes,
    InvalidLiteralForRadix,
    MissingTrait,
    RedefinitionAttempt,
    NameNotFound,
    NestedFunctions,
    CannotAssign,
    UnknownType,
    ArgumentCountMismatch,
    TypeMismatch,
    ImmutableAssign,
    InvalidCast,
    NameNotOwned,
    ReturnValueNotOwned,
    ReturnOutsideOfFunction,
    StructNotDefined,
    InvalidMemberCount,
    MemberNameNotFound,
    FieldRedeclaration,
    FieldReinitialization,
    GetAttrOfNonStruct,
    StructAttrNotFound,
    CannotDefineVoidArray,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorType::InvalidTok => write!(f, "invalid token"),
            ErrorType::InvalidDataTypes => write!(f, "invalid data types for operation"),
            ErrorType::InvalidLiteralForRadix => write!(f, "invalid data literal for implicit or explicit radix"),
            ErrorType::MissingTrait => write!(f, "missing trait"),
            ErrorType::RedefinitionAttempt => write!(f, "attempt to redefine name"),
            ErrorType::NameNotFound => write!(f, "name not defined"),
            ErrorType::NestedFunctions => write!(f, "attempt to define nested functions"),
            ErrorType::CannotAssign => write!(f, "cannot assign to type"),
            ErrorType::UnknownType => write!(f, "unknown type"),
            ErrorType::ArgumentCountMismatch => write!(f, "invalid number of arguments were passed to a function"),
            ErrorType::TypeMismatch => write!(f, "mismatch of types"),
            ErrorType::ImmutableAssign => write!(f, "cannot assign to immutable variable"),
            ErrorType::InvalidCast => write!(f, "invalid cast"),
            ErrorType::NameNotOwned => write!(f, "name is not owned"),
            ErrorType::ReturnValueNotOwned => write!(f, "return value is not owned"),
            ErrorType::ReturnOutsideOfFunction => write!(f, "cannot return outside of function"),
            ErrorType::StructNotDefined => write!(f, "struct is not defined"),
            ErrorType::InvalidMemberCount => write!(f, "invalid member count"),
            ErrorType::MemberNameNotFound => write!(f, "member name not found"),
            ErrorType::FieldRedeclaration => write!(f, "field is redefined"),
            ErrorType::FieldReinitialization => write!(f, "field is reinitialized"),
            ErrorType::GetAttrOfNonStruct => write!(f, "cannot get attribute of non-struct"),
            ErrorType::StructAttrNotFound => write!(f, "attribute not found"),
            ErrorType::CannotDefineVoidArray => write!(f, "cannot define void array"),
        }
    }
}

pub fn raise_error(error: &str, errtp: ErrorType, pos: &crate::parser::Position, info: &crate::fileinfo::FileInfo) -> !{
    let header: String = format!("error[E{:0>3}]: {}", errtp as u8 + 1, error);
    let location: String = format!("{}:{}:{}", info.name, pos.line+1, pos.startcol+1);
    println!("{}", header.red().bold());
    println!("{}", location.red());
    let lines = Vec::from_iter(info.data.split(|num| *num as char == '\n'));

    let snippet: String = format!("{}", String::from_utf8(lines.get(pos.line).unwrap().to_vec()).unwrap().blue());
    let mut arrows: String = String::new();
    for idx in 0..snippet.len() {
        if (idx as usize)>=pos.startcol && (idx as usize)<pos.endcol {
            arrows += "^";
        }
        else {
            arrows += " ";
        }
    }
    let linestr = (pos.line+1).to_string().blue().bold();
    println!("{} | {}", linestr, snippet);
    println!("{} | {}", " ".repeat(linestr.len()), arrows.green());
    std::process::exit(1);
}

pub fn raise_error_no_pos(error: &str, errtp: ErrorType) -> !{
    let header: String = format!("error[E{:0>3}]: {}", errtp as u8 + 1, error);
    println!("{}", header.red().bold());
    std::process::exit(1);
}

pub fn raise_error_multi(errtp: ErrorType, err: Vec<String>, pos: Vec<&crate::parser::Position>, info: &crate::fileinfo::FileInfo) -> !{
    let mut idx: usize = 0;
    for (error, pos) in std::iter::zip(&err, pos) {
        let location: String = format!("{}:{}:{}", info.name, pos.line+1, pos.startcol+1);
        if idx != err.len()-1 {
            let header: String = format!("{}", error);
            println!("{}", header.yellow().bold());
        }
        else {
            let header: String = format!("error[E{:0>3}]: {}", errtp.clone() as u8 + 1, error);
            println!("{}", header.red().bold());
        }
        idx += 1;
        println!("{}", location.red());
        let lines = Vec::from_iter(info.data.split(|num| *num as char == '\n'));

        let snippet: String = format!("{}", String::from_utf8(lines.get(pos.line).unwrap().to_vec()).unwrap().blue());
        let mut arrows: String = String::new();
        for idx in 0..snippet.len() {
            if (idx as usize)>=pos.startcol && (idx as usize)<pos.endcol {
                arrows += "^";
            }
            else {
                arrows += " ";
            }
        }
        let linestr = (pos.line+1).to_string().blue().bold();
        println!("{} | {}", linestr, snippet);
        println!("{} | {}", " ".repeat(linestr.len()), arrows.green());
    }
    std::process::exit(1);
}