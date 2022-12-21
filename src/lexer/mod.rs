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
    COLON,
    COMMA,
    SMALLARROW,
    U32,
    I8,
    U8,
    I16,
    U16,
    I64,
    U64,
    I128,
    U128,
    UNKNOWN,
}

pub struct Lexer<'life> {
    pub idx: usize,
    pub data: &'life [u8],
    pub current: char,
    pub len: usize,
    pub line: usize,
    pub col: usize,
    pub info: &'life crate::fileinfo::FileInfo<'life>,
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
           TokenType::COLON => write!(f, "COLON"),
           TokenType::COMMA => write!(f, "COMMA"),
           TokenType::SMALLARROW => write!(f, "SMALLARROW"),
           TokenType::U32 => write!(f, "u32"),
           TokenType::I8 => write!(f, "i8"),
           TokenType::U8 => write!(f, "u8"),
           TokenType::I16 => write!(f, "i16"),
           TokenType::U16 => write!(f, "u16"),
           TokenType::I64 => write!(f, "i64"),
           TokenType::U64 => write!(f, "u64"),
           TokenType::I128 => write!(f, "i128"),
           TokenType::U128 => write!(f, "u128"),
           TokenType::UNKNOWN => write!(f, "UNKNOWN"),
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

    while lexer.current!='\0' {
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
            if lexer.current == '>' {     
                let popped: Token = vector.pop().unwrap();           
                vector.push(Token {
                    data: String::from("->"),
                    tp: TokenType::SMALLARROW,
                    line: popped.line,
                    startcol: popped.startcol,
                    endcol: popped.endcol+1,
                });
                advance(lexer);
            }
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
        else if cur == ':' {
            vector.push(Token {
                data: String::from(":"),
                tp: TokenType::COLON,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == ',' {
            vector.push(Token {
                data: String::from(","),
                tp: TokenType::COMMA,
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
        else if cur.is_whitespace() {
            advance(lexer);
        }
        else {
            vector.push(Token {
                data: String::from(cur),
                tp: TokenType::UNKNOWN,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }

    }

    vector.push(Token {
        data: String::from("\\0"),
        tp: TokenType::EOF,
        line: lexer.line,
        startcol: lexer.col,
        endcol: lexer.col+1,
    });

    return (vector.len(), vector);
}

fn make_number(lexer: &mut Lexer) -> Token {
    let mut data: String = String::from("");
    let start: usize = lexer.col;

    let mut end: usize = lexer.col;
    let mut line: usize = lexer.line;

    let mut tp: TokenType = TokenType::I32;

    while lexer.current.is_numeric() || lexer.current=='_' {
        data.push(lexer.current);
        end=lexer.col;
        line=lexer.line;
        advance(lexer);
        if lexer.current == 'u' || lexer.current == 'i' {
            let mut specified_tp: String = String::from(lexer.current);
            advance(lexer);
            while lexer.current.is_numeric() {
                specified_tp.push(lexer.current.to_ascii_lowercase());
                end=lexer.col;
                line=lexer.line;
                advance(lexer);
            }

            if specified_tp==crate::codegen::types::BasicDataType::I32.to_string() {
                tp=TokenType::I32;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::U32.to_string() {
                tp=TokenType::U32;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::I8.to_string() {
                tp=TokenType::I8;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::U8.to_string() {
                tp=TokenType::U8;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::I16.to_string() {
                tp=TokenType::I16;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::U16.to_string() {
                tp=TokenType::U16;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::I64.to_string() {
                tp=TokenType::I64;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::U64.to_string() {
                tp=TokenType::U64;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::I128.to_string() {
                tp=TokenType::I128;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::U128.to_string() {
                tp=TokenType::U128;
            }
            else {
                crate::errors::raise_error(format!("Invalid specified type {}.", specified_tp).as_str(), crate::errors::ErrorType::UnknownType, &crate::parser::Position { line, startcol: start, endcol: end+1 }, lexer.info);
            }

            break;            
        }
    }
    
    let tok = Token {
        data: data,
        tp,
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

    while lexer.current.is_alphabetic() || lexer.current=='_' || lexer.current.is_numeric(){
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