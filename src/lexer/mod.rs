//Generate tokens from text

#[derive(Clone, PartialEq, Debug)]
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
    F32,
    F64,
    AMPERSAND,
    DOT,
    STRING,
    LSQUARE,
    RSQUARE,
    CHAR,
    EMOJIERR,
    DOUBLECOLON,
}

pub struct Lexer<'life> {
    pub idx: usize,
    pub data: &'life [u8],
    pub current: u8,
    pub len: usize,
    pub line: usize,
    pub col: usize,
    pub info: &'life crate::fileinfo::FileInfo<'life>,
}

#[derive(Clone, Debug)]
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
           TokenType::F32 => write!(f, "f32"),
           TokenType::F64 => write!(f, "f64"),
           TokenType::AMPERSAND => write!(f, "AMPERSAND"),
           TokenType::DOT => write!(f, "DOT"),
           TokenType::STRING => write!(f, "STRING"),
           TokenType::LSQUARE => write!(f, "LSQUARE"),
           TokenType::RSQUARE => write!(f, "RSQUARE"),
           TokenType::CHAR => write!(f, "CHAR"),
           TokenType::EMOJIERR => write!(f, "EMOJIERR"),
           TokenType::DOUBLECOLON => write!(f, "DOUBLECOLON"),
       }
    }
}

fn advance(lexer: &mut Lexer) {
    lexer.idx+=1;

    lexer.col+=1;

    if lexer.idx >= lexer.len {
        lexer.current = b'\0';
        return;
    }
    
    lexer.current = lexer.data[lexer.idx];

    if lexer.current == b'\n' || lexer.current == b'\r' {
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
    let mut tokens: Vec<Token> = Vec::new();

    while lexer.current!=b'\0' {
        let cur: char = lexer.current.into();
        
        if cur.is_digit(10) {
            tokens.push(make_number(lexer));
        }
        else if cur.is_alphabetic() || cur=='_'{
            tokens.push(make_identifier(lexer, kwds));
        }
        else if cur=='"'{
            tokens.push(make_string(lexer));
        }
        else if cur=='\''{
            tokens.push(make_char(lexer));
        }
        else if cur == '+' {
            tokens.push(Token {
                data: String::from("+"),
                tp: TokenType::PLUS,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '-' {
            tokens.push(Token {
                data: String::from("-"),
                tp: TokenType::HYPHEN,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
            if lexer.current == b'>' {     
                let popped: Token = tokens.pop().unwrap();           
                tokens.push(Token {
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
            tokens.push(Token {
                data: String::from("*"),
                tp: TokenType::ASTERISK,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '/' {
            tokens.push(Token {
                data: String::from("/"),
                tp: TokenType::FWSLASH,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
            if lexer.current == '/' as u8 {
                tokens.pop();
                while lexer.current != '\n' as u8 && lexer.current != '\0' as u8 {
                    advance(lexer);
                }
            }
        }
        else if cur == '=' {
            tokens.push(Token {
                data: String::from("="),
                tp: TokenType::EQUALS,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '{' {
            tokens.push(Token {
                data: String::from("{"),
                tp: TokenType::LCURLY,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '}' {
            tokens.push(Token {
                data: String::from("}"),
                tp: TokenType::RCURLY,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '(' {
            tokens.push(Token {
                data: String::from("("),
                tp: TokenType::LPAREN,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == ')' {
            tokens.push(Token {
                data: String::from(")"),
                tp: TokenType::RPAREN,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == ':' {
            tokens.push(Token {
                data: String::from(":"),
                tp: TokenType::COLON,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
            if lexer.current == b':' {     
                let popped: Token = tokens.pop().unwrap();           
                tokens.push(Token {
                    data: String::from("::"),
                    tp: TokenType::DOUBLECOLON,
                    line: popped.line,
                    startcol: popped.startcol,
                    endcol: popped.endcol+1,
                });
                advance(lexer);
            }
        }
        else if cur == ',' {
            tokens.push(Token {
                data: String::from(","),
                tp: TokenType::COMMA,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == ';' as char || cur == '\r' as char || cur == '\n' as char {
            tokens.push(Token {
                data: String::from("\\n"),
                tp: TokenType::NEWLINE,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
            lexer.col = 0;
        }
        else if cur == '&' {
            tokens.push(Token {
                data: String::from("&"),
                tp: TokenType::AMPERSAND,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '.' {
            tokens.push(Token {
                data: String::from("."),
                tp: TokenType::DOT,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == '[' {
            tokens.push(Token {
                data: String::from("["),
                tp: TokenType::LSQUARE,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur == ']' {
            tokens.push(Token {
                data: String::from("]"),
                tp: TokenType::RSQUARE,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }
        else if cur.is_whitespace() {
            advance(lexer);
        }
        else {
            tokens.push(Token {
                data: String::from(cur),
                tp: TokenType::UNKNOWN,
                line: lexer.line,
                startcol: lexer.col,
                endcol: lexer.col+1,
            });
            advance(lexer);
        }

    }

    tokens.push(Token {
        data: String::from("\\0"),
        tp: TokenType::EOF,
        line: lexer.line,
        startcol: lexer.col,
        endcol: lexer.col+1,
    });

    return (tokens.len(), tokens);
}

fn make_number(lexer: &mut Lexer) -> Token {
    let mut data: String = String::from("");
    let start: usize = lexer.col;

    let mut end: usize = lexer.col;
    let mut line: usize = lexer.line;

    let mut tp: TokenType = TokenType::I32;

    while (lexer.current as char).is_numeric() || lexer.current==b'_' {
        data.push(lexer.current as char);
        end=lexer.col;
        line=lexer.line;
        advance(lexer);
        if lexer.current == b'.' {
            tp=TokenType::F32;
            data.push(lexer.current as char);
            advance(lexer);
        }
        if lexer.current == b'f' {            
            let mut specified_tp: String = String::from(lexer.current as char);
            advance(lexer);
            while (lexer.current as char).is_numeric() {
                specified_tp.push((lexer.current as char).to_ascii_lowercase());
                end=lexer.col;
                line=lexer.line;
                advance(lexer);
            }

            if specified_tp==crate::codegen::types::BasicDataType::F32.to_string() {
                tp=TokenType::F32;
            }
            else if specified_tp==crate::codegen::types::BasicDataType::F64.to_string() {
                tp=TokenType::F64;
            }
            else {
                crate::errors::raise_error(format!("Invalid specified type {}.", specified_tp).as_str(), crate::errors::ErrorType::UnknownType, &crate::parser::Position { line, startcol: start, endcol: end+1 }, lexer.info);
            }
        }
        else if lexer.current == b'u' || lexer.current == b'i' {
            let mut specified_tp: String = String::from(lexer.current as char);
            advance(lexer);
            while (lexer.current as char).is_numeric() {
                specified_tp.push((lexer.current as char).to_ascii_lowercase());
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
            else if specified_tp=="usize" { 
                if std::mem::size_of::<usize>() == std::mem::size_of::<u32>() {
                    tp=TokenType::U32;   
                }
                else {
                    tp=TokenType::U64;
                }
            }
            else if specified_tp=="isize" { 
                if std::mem::size_of::<isize>() == std::mem::size_of::<i32>() {
                    tp=TokenType::I32;   
                }
                else {
                    tp=TokenType::I64;
                }
            }
            else if specified_tp=="bool" { 
                tp=TokenType::I8;
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
    let mut data: Vec<u8> = Vec::new();
    let start: usize = lexer.col;

    let mut end: usize = lexer.col;
    let mut line: usize = lexer.line;

    while (lexer.current as char).is_alphabetic() || lexer.current==b'_' || (lexer.current as char).is_numeric(){
        data.push(lexer.current);
        end=lexer.col;
        line=lexer.line;
        advance(lexer);
    }

    if String::from_utf8(data.clone()).is_err() {
        for _ in 0..3 {
            data.push(lexer.current);
            advance(lexer);
        }
        let tok = Token {
            data: String::from_utf8(data.clone()).unwrap(),
            tp: TokenType::EMOJIERR,
            line,
            startcol: start,
            endcol: start+1+unicode_width::UnicodeWidthChar::width(String::from_utf8(data.clone()).unwrap().chars().nth(0).unwrap()).unwrap(),
        };
        return tok;
    }
    
    let mut tok = Token {
        data: if data.len() > 0 {String::from_utf8(data).unwrap()} else {String::from("")},
        tp: TokenType::IDENTIFIER,
        line,
        startcol: start,
        endcol: end+1,
    };

    if kwds.iter().find(|x| **x==tok.data)!=None {
        tok.tp = TokenType::KEYWORD;
        if  tok.data == String::from("true") {
            tok = Token {
                data: String::from("1"),
                tp: TokenType::I8,
                line,
                startcol: start,
                endcol: end+1,
            };                
        }
        else if  tok.data == String::from("false") {
            tok = Token {
                data: String::from("0"),
                tp: TokenType::I8,
                line,
                startcol: start,
                endcol: end+1,
            };                
        }
    }
    return tok;
}

fn make_string(lexer: &mut Lexer) -> Token {
    let mut data: Vec<u8> = Vec::new();
    let start: usize = lexer.col;

    let line: usize = lexer.line;

    advance(lexer);

    while lexer.current!=b'"'{
        data.push(lexer.current);
        advance(lexer);
    }

    let mut end: usize = start+1;

    if data.len() > 0 {
        for itm in String::from_utf8(data.clone()).unwrap().chars() {
            end += unicode_width::UnicodeWidthChar::width(itm).unwrap();
        }
    }

    lexer.col = end;
    
    let tok = Token {
        data: if data.len() > 0 {String::from_utf8(data.clone()).unwrap()} else {String::from("")},
        tp: TokenType::STRING,
        line,
        startcol: start,
        endcol: end+1,
    };

    
    advance(lexer);
    
    return tok;
}

fn make_char(lexer: &mut Lexer) -> Token {
    let mut data: Vec<u8> = Vec::new();
    let start: usize = lexer.col;

    let mut line: usize = lexer.line;

    advance(lexer);

    while lexer.current!=b'\''{
        data.push(lexer.current);
        line=lexer.line;
        advance(lexer);
    }

    let mut end: usize = start+1;

    if data.len() > 0 {
        for itm in String::from_utf8(data.clone()).unwrap().chars() {
            end += unicode_width::UnicodeWidthChar::width(itm).unwrap();
        }
    }

    lexer.col = end;
    
    let tok = Token {
        data: if data.len() > 0 {String::from_utf8(data).unwrap()} else {String::from("")},
        tp: TokenType::CHAR,
        line,
        startcol: start,
        endcol: end+1,
    };
    
    advance(lexer);
    
    return tok;
}