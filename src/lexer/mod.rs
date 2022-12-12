//Generate tokens from text

pub enum TokenType {
    INT,
    PLUS,
    NEWLINE,
}

pub struct Lexer <'life> {
    pub idx: usize,
    pub data: &'life [u8],
    pub current: char,
    pub len: usize,
}

pub struct Token {
    pub data: String,
    pub tp: TokenType,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: '{}'", self.tp, self.data)
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       match *self {
           TokenType::INT => write!(f, "INT"),
           TokenType::PLUS => write!(f, "PLUS"),
           TokenType::NEWLINE => write!(f, "NEWLINE"),
       }
    }
}

fn advance(lexer: &mut Lexer) {
    lexer.idx+=1;

    if lexer.idx == lexer.len {
        lexer.current = '\0';
        return;
    }
    
    lexer.current = lexer.data[lexer.idx] as char;
}

pub fn print_tokens(len: usize, tokens: Vec<Token>){
    println!("========================");
    println!("Token list ({} tokens)", len);
    println!("------------------------");
    let mut idx: usize = 1;
    for tok in tokens{
        let repr: String = format!("{} | {}", idx, tok);
        println!("| {:<21}|", repr);
        idx+=1;
    }
    println!("========================");
}

pub fn generate_tokens(mut lexer: Lexer) -> (usize, Vec<Token>) {  
    let mut vector: Vec<Token> = Vec::new();

    for _ in 0 .. lexer.data.len() {
        let cur: char = lexer.current;
        
        if cur.is_numeric() {
            vector.push(make_number(&mut lexer));
        }
        else if cur == '+' {
            vector.push(Token {
                data: String::from("+"),
                tp: TokenType::PLUS,
            });
            advance(&mut lexer);
        }
        else if cur == '\r' as char || cur == '\n' as char {
            vector.push(Token {
                data: String::from("\\n"),
                tp: TokenType::NEWLINE,
            });
            advance(&mut lexer);
            if lexer.current=='\n' as char { // Windows compat
                advance(&mut lexer);
            }
        }
    }

    return (vector.len(), vector);
}

fn make_number(lexer: &mut Lexer) -> Token {
    let mut data: String = String::from("");

    while lexer.current.is_numeric() {
        data.push(lexer.current);
        advance(lexer);
    }
    
    let tok = Token {
        data: data,
        tp: TokenType::INT,
    };
    return tok;
}