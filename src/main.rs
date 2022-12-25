use fileinfo::FileInfo;

mod fileinfo;
mod errors;
mod lexer;
mod parser;
mod codegen;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len()!=2 {
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
                                            ];

    let mut lexer: lexer::Lexer = lexer::Lexer {
        idx: 0,
        data: file_data_bytes,
        current: file_data_bytes[0] as char,
        len: file_data_bytes.len(),
        line: 0,
        col: 0,
        info: &file_info,
    };

    let (_, tokens) = lexer::generate_tokens(&mut lexer, &mut keywords);

    lexer::print_tokens(tokens.len(), &tokens);

    let mut parser: parser::Parser = parser::Parser {
        tokens: &tokens,
        idx: 1,
        current: tokens.first().unwrap().to_owned(),
        info: &file_info,
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