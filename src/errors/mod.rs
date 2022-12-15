use colored::Colorize;

pub enum ErrorType {
    InvalidTok,    
    InvalidDataTypes,
    InvalidLiteralForRadix,
    MissingTrait,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorType::InvalidTok => write!(f, "invalid token"),
            ErrorType::InvalidDataTypes => write!(f, "invalid data types for operation"),
            ErrorType::InvalidLiteralForRadix => write!(f, "invalid data literal for implicit or explicit radix"),
            ErrorType::MissingTrait => write!(f, "missing trait"),
        }
    }
}

pub fn raise_error(error: &str, errtp: ErrorType, pos: &crate::parser::Position, info: &crate::fileinfo::FileInfo) -> !{
    let header: String = format!("error[E{:0>5}]: {}", errtp as u8 + 1, error);
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