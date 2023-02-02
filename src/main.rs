use fileinfo::FileInfo;
extern crate num;
#[macro_use]
extern crate num_derive;

mod fileinfo;
mod errors;
mod lexer;
mod parser;
mod codegen;

mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.last().unwrap() == &String::from("--help") && args.len() == 2 {
        println!("usage: kestrel [--version | --help] [--err <error> | --warn <warning>] [<program>]");
        return;
    }

    if args.last().unwrap() == &String::from("--version") && args.len() == 2 {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.get(1).unwrap() == &String::from("--err") {
        let error = args.get(2).unwrap().parse::<i32>();
        if error.is_err() || error.clone().unwrap()<=0{
            println!("Invalid numeric value.");
            return;
        }

        let res: Option<errors::ErrorType> = num::FromPrimitive::from_i32(error.clone().unwrap()-1);
        
        match res {
            Some(v) => {
                println!("{}", format!("error[E{:0>3}]: {}", error.unwrap(), errors::repr_err(v)));
            }
            None => {
                println!("Error not found.");
            }
        }
        return;
    }

    if args.get(1).unwrap() == &String::from("--warn") {
        let warning = args.get(2).unwrap().parse::<i32>();
        if warning.is_err() || warning.clone().unwrap()<=0 {
            println!("Invalid numeric value.");
            return;
        }

        let res: Option<errors::WarningType> = num::FromPrimitive::from_i32(warning.clone().unwrap()-1);
        
        match res {
            Some(v) => {
                println!("{}", format!("warning[E{:0>3}]: {}", warning.unwrap(), errors::repr_warn(v)));
            }
            None => {
                println!("Error not found.");
            }
        }
        return;        
    }

    if args.len() != 2 {
        println!("Invalid number of command line arguments. Expected 2, got {}.", args.len());
        return;
    }

    let filename: &String = &args[1];
    let file_data: String;
    
    
    let res: Result<String, std::io::Error> = std::fs::read_to_string(filename);
    match res {
        Ok(_) => {
            file_data = res.unwrap();
        }
        Err(_) => {
            println!("File '{}' is unable to be opened or read.", filename);
            return;
        }
    }

    let file_data_bytes: &[u8] = file_data.as_bytes();

    let file_info: FileInfo = FileInfo {
        data: file_data_bytes.clone(),
        name: filename.clone(),
        dir: String::from("."),
    };

    let mut keywords: Vec<String> = vec![   String::from("let"),
                                            String::from("fn"),
                                            String::from("mut"),
                                            String::from("return"),
                                            String::from("to"),
                                            String::from("as"),
                                            String::from("struct"),
                                            String::from("impl"),
                                            String::from("for"),
                                            String::from("true"),
                                            String::from("false"),
                                            String::from("if"),
                                            String::from("elif"),
                                            String::from("else"),
                                            String::from("loop"),
                                            String::from("break"),
                                            String::from("continue"),
                                            String::from("while"),
                                            String::from("enum"),
                                            String::from("trait"),
                                            String::from("dyn"),
                                            String::from("void"),
                                            String::from("is"),
                                            String::from("match"),
                                            ];

    let mut lexer: lexer::Lexer = lexer::Lexer {
        idx: 0,
        data: file_data_bytes,
        current: file_data_bytes[0],
        len: file_data_bytes.len(),
        line: 0,
        col: 0,
        info: &file_info,
    };

    let (_, tokens) = lexer::generate_tokens(&mut lexer, &mut keywords);

    //lexer::print_tokens(tokens.len(), &tokens);

    let mut parser: parser::Parser = parser::Parser {
        tokens: &tokens,
        idx: 1,
        current: tokens.first().unwrap().to_owned(),
        info: &file_info,
        allow_init: parser::StructConstructionAllowance::new(),
    };

    let nodes: Vec<parser::Node> = parser.genreate_ast();

    let res: Result<(), Box<dyn std::error::Error>> = codegen::generate_code("module", filename.as_str(), nodes, &file_info);

    match res {
        Ok(_) => {}

        Err(err) => {
            panic!("{}",err.to_string());
        }
    }

}