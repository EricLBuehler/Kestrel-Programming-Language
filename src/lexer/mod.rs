//Generate tokens from text

#[derive(Clone, PartialEq)]
pub enum TokenType {
    I32,
    PLUS,
    NEWLINE,
    EOF,
    ASTERISK,
    FWSLASH,
    HYPHEN,
    KEYWORD,
    IDENTIFIER,
    EQUALS,
    LCURLY,
    RCURLY,
    LPAREN,
    RPAREN,
}

pub struct Lexer<'life> {
    pub idx: usize,
    pub data: &'life [u8],
    pub current: char,
    pub len: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct Token {
    pub data: String,
    pub tp: TokenType,
    pub line: usize,
    pub startcol: usize, //Inclusive
    pub endcol: usize, //Exclusive
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: '{}'", self.tp, self.data)
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       match *self {
           TokenType::I32 => write!(f, "i32"),
           TokenType::PLUS => write!(f, "PLUS"),
           TokenType::NEWLINE => write!(f, "NEWLINE"),
           TokenType::EOF => write!(f, "EOF"),
           TokenType::ASTERISK => write!(f, "ASTERISK"),
           TokenType::FWSLASH => write!(f, "FWSLASH"),
           TokenType::HYPHEN => write!(f, "HYPHEN"),
           TokenType::KEYWORD => write!(f, "KEYWORD"),
           TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
           TokenType::EQUALS => write!(f, "EQUALS"),
           TokenType::LCURLY => write!(f, "LCURLY"),
           TokenType::RCURLY => write!(f, "RCURLY"),
           TokenType::LPAREN => write!(f, "LPAREN"),
           TokenType::RPAREN => write!(f, "RPAREN"),
       }
    }
}

fn advance(lexer: &mut Lexer) {
    lexer.idx+=1;

    lexer.col+=1;

    if lexer.idx >= lexer.len {
        lexer.current = '\0';
        return;
    }
    
    lexer.current = lexer.data[lexer.idx] as char;

    if lexer.current == '\n' || lexer.current == '\r' {
        lexer.line+=1;
        lexer.col=0;
    }
}

#[allow(dead_code)]
pub fn print_tokens(len: usize, tokens: &Vec<Token>) {
    println!("\n\nGenerated tokens:\n========================");
    println!("Token list ({} tokens)", len);
    println!("------------------------");
    let mut idx: usize = 1;
    for tok in tokens{
        println!("{} | {}", idx, tok);
        idx+=1;
    }
    println!("========================");
}

pub fn generate_tokens(lexer: &mut Lexer, kwds: &Vec<String>) -> (usize, Vec<Token>) {  
    let mut vector: Vec<Token> = Vec::new();

    for _ in 0 .. lexer.data.len() {
        let cur: char = lexer.current;
        
        if cur.is_digit(10) {
            vector.push(make_number(lexer));
        }
        else if cur.is_alphabetic() || cur=='_'{
            vector.push(make_identifier(lexer, kwds));
        }
        else if cur == '+' {
            vector.push(Token {
                data: String::from("+"),
                tp: TokenType::PLUS,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '-' {
            vector.push(Token {
                data: String::from("-"),
                tp: TokenType::HYPHEN,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);

        }
        else if cur == '*' {
            vector.push(Token {
                data: String::from("*"),
                tp: TokenType::ASTERISK,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '/' {
            vector.push(Token {
                data: String::from("/"),
                tp: TokenType::FWSLASH,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '=' {
            vector.push(Token {
                data: String::from("="),
                tp: TokenType::EQUALS,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '{' {
            vector.push(Token {
                data: String::from("{"),
                tp: TokenType::LCURLY,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '}' {
            vector.push(Token {
                data: String::from("}"),
                tp: TokenType::RCURLY,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '(' {
            vector.push(Token {
                data: String::from("("),
                tp: TokenType::LPAREN,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == ')' {
            vector.push(Token {
                data: String::from(")"),
                tp: TokenType::RPAREN,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == ';' as char || cur == '\r' as char || cur == '\n' as char {
            vector.push(Token {
                data: String::from("\\n"),
                tp: TokenType::NEWLINE,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
            lexer.col = 0;
        }
        else {
            advance(lexer);
        }

    }

    vector.push(Token {
        data: String::from("\\0"),
        tp: TokenType::EOF,
        line: lexer.line,
        startcol: lexer.col,
        endcol: lexer.col,
    });

    return (vector.len(), vector);
}

fn make_number(lexer: &mut Lexer) -> Token {
    let mut data: String = String::from("");
    let start: usize = lexer.col;

    let mut end: usize = lexer.col;
    let mut line: usize = lexer.line;

    while lexer.current.is_numeric() || lexer.current=='_' {
        data.push(lexer.current);
        end=lexer.col;
        line=lexer.line;
        advance(lexer);
    }
    
    let tok = Token {
        data: data,
        tp: TokenType::I32,
        line,
        startcol: start,
        endcol: end+1,
    };
    return tok;
}

fn make_identifier(lexer: &mut Lexer, kwds: &Vec<String>) -> Token {
    let mut data: String = String::from("");
    let start: usize = lexer.col;

    let mut end: usize = lexer.col;
    let mut line: usize = lexer.line;

    while lexer.current.is_alphabetic() || lexer.current=='_' {
        data.push(lexer.current);
        end=lexer.col;
        line=lexer.line;
        advance(lexer);
    }
    
    let mut tok = Token {
        data: data,
        tp: TokenType::IDENTIFIER,
        line,
        startcol: start,
        endcol: end+1,
    };

    if kwds.iter().find(|x| **x==tok.data)!=None {
        tok.tp = TokenType::KEYWORD;
    }
    return tok;
}