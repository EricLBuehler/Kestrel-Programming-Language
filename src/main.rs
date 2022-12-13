mod lexer;
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

    let lexer: lexer::Lexer = lexer::Lexer{
        idx: 0,
        data: file_data_bytes,
        current: file_data_bytes[0] as char,
        len: file_data_bytes.len(),
    };

    let (ntok, tokens) = lexer::generate_tokens(lexer);

    lexer::print_tokens(ntok, tokens);

    let res: Result<(), Box<dyn std::error::Error>> = codegen::generate_code("module", filename.as_str());

    match res {
        Ok(_) => {}

        Err(err) => {
            panic!("{}",err.to_string());
        }
    }

}