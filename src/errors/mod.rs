use colored::Colorize;

#[derive(Clone, FromPrimitive)]
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
    UnexpectedMultibyte,
    ArrayLengthOutOfRange,
    CannotDefineFnArray,
    ZeroLengthArray,
    EmptyCharLiteral,
    GlobalScopeStmt,
    ImmutableAttr,
    InvalidStatement,
    TraitNotFound,
    TraitExpectProperFunctionName,
    CannotImplementCallTrait,
    NamespaceAttrNotFound,
    TypeRedefinitionAttempt,
    ExpectedSpecifiedType,
    NameNotInitialized,
    BreakOutsideOfLoop,
    ContinueOutsideOfLoop,
    VariantRedeclaration,
    MethodTemplateFunctionHasFirstTemplate,
    UnknownTemplateType,
    LocalScopeStmt,
    CannotImplementBuiltinTrait,
    ExpectedNFunctionsDefined,
    ImplFunctionTemplateTypeMismatch,
    FunctionNotDefinedInTrait,
    FunctionRedefinedInImpl,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", repr_err(self.clone()))
    }
}

pub fn repr_err(tp: ErrorType) -> &'static str {
    match tp {
        ErrorType::InvalidTok => "invalid token encountered",
        ErrorType::InvalidDataTypes => "invalid data types for operation",
        ErrorType::InvalidLiteralForRadix => "invalid data literal for implicit or explicit radix",
        ErrorType::MissingTrait => "missing trait",
        ErrorType::RedefinitionAttempt => "attempt to redefine name",
        ErrorType::NameNotFound => "name not defined",
        ErrorType::NestedFunctions => "attempt to define nested functions",
        ErrorType::CannotAssign => "cannot assign to type",
        ErrorType::UnknownType => "unknown type",
        ErrorType::ArgumentCountMismatch => "invalid number of arguments were passed to a function",
        ErrorType::TypeMismatch => "mismatch of types",
        ErrorType::ImmutableAssign => "cannot assign to immutable variable",
        ErrorType::InvalidCast => "invalid cast",
        ErrorType::NameNotOwned => "name is not owned",
        ErrorType::ReturnValueNotOwned => "return value is not owned",
        ErrorType::ReturnOutsideOfFunction => "cannot return outside of function",
        ErrorType::StructNotDefined => "struct is not defined",
        ErrorType::InvalidMemberCount => "invalid member count for struct initialization",
        ErrorType::MemberNameNotFound => "member name not found in struct",
        ErrorType::FieldRedeclaration => "field is redefined in struct initialization",
        ErrorType::FieldReinitialization => "field is reinitialized",
        ErrorType::GetAttrOfNonStruct => "cannot get attribute of non-struct type",
        ErrorType::StructAttrNotFound => "attribute not found in struct type",
        ErrorType::CannotDefineVoidArray => "cannot define array of void",
        ErrorType::UnexpectedMultibyte => "unexpected multibyte character in 'char' literal",
        ErrorType::ArrayLengthOutOfRange => "array length out of range of 'u32'",
        ErrorType::CannotDefineFnArray => "cannot define array of 'fn'",
        ErrorType::ZeroLengthArray => "cannot define zero length array",
        ErrorType::EmptyCharLiteral => "char literals cannot be empty",
        ErrorType::GlobalScopeStmt => "cannot have other statements in the global scope",
        ErrorType::ImmutableAttr => "immutable attr assign",
        ErrorType::InvalidStatement => "immutable statement",
        ErrorType::TraitNotFound => "trait not found",
        ErrorType::TraitExpectProperFunctionName => "trait implementation expects function with proper name",
        ErrorType::CannotImplementCallTrait => "cannot implement call trait",
        ErrorType::NamespaceAttrNotFound => "namespace attribute is not found",
        ErrorType::TypeRedefinitionAttempt => "attempt to redefine type",
        ErrorType::ExpectedSpecifiedType => "expected specified type",
        ErrorType::NameNotInitialized => "name is not necessarily initialized",
        ErrorType::BreakOutsideOfLoop => "cannot break outside of loop",
        ErrorType::ContinueOutsideOfLoop => "cannot continue outside of loop",
        ErrorType::VariantRedeclaration => "variant is refedined in enum",
        ErrorType::MethodTemplateFunctionHasFirstTemplate => "template method cannot have template first argument",
        ErrorType::UnknownTemplateType => "unknown template type",
        ErrorType::LocalScopeStmt => "invalic local scope statement",
        ErrorType::CannotImplementBuiltinTrait => "cannot implement builtin trait",
        ErrorType::ExpectedNFunctionsDefined => "expected n functions defined in implementation",
        ErrorType::ImplFunctionTemplateTypeMismatch => "template argument type mismatch in impl",
        ErrorType::FunctionNotDefinedInTrait => "function is not defined in trait impl",
        ErrorType::FunctionRedefinedInImpl => "function is redefined in trait impl",
    }
}


#[derive(Clone, FromPrimitive)]
pub enum WarningType {
    ExpectedCamelCase,
    ExpectedSnakeCase,
    UnreachableCode,
}

impl std::fmt::Display for WarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", repr_warn(self.clone()))
    }
}

pub fn repr_warn(tp: WarningType) -> &'static str {
    match tp {
        WarningType::ExpectedCamelCase => "expected camel case",
        WarningType::ExpectedSnakeCase => "expected snake case",
        WarningType::UnreachableCode => "unreachable code",
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

pub fn show_warning(warntp: WarningType, text: Vec<String>, warning: Vec<String>, pos: &crate::parser::Position, info: &crate::fileinfo::FileInfo){
    let mut idx: usize = 0;
    for (warn, text) in std::iter::zip(&warning, text) {
        let location: String = format!("{}:{}:{}", info.name, pos.line+1, pos.startcol+1);
        if idx != 0 {
            let header: String = format!("{}", warn);
            println!("{}", header.bright_yellow().bold());
            idx += 1;
            let txt: String = format!("{}", text);
            let linestr = (pos.line+1).to_string().blue().bold();
            println!("{} | {}", linestr, txt.green());
        }
        else {
            let header: String = format!("warning[W{:0>3}]: {}", warntp.clone() as u8 + 1, warn);
            println!("{}", header.bright_yellow().bold());
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
    }

    println!("");
}