#[cfg(test)]

#[test]
fn test_lexer() {
    let file_data: String = String::from("fn main(){Aa\n123\n\"🦅\"}");
    let filename: &String = &String::from("file.ke");

    let file_data_bytes: &[u8] = file_data.as_bytes();

    let file_info: crate::fileinfo::FileInfo = crate::fileinfo::FileInfo {
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
                                            ];

    let mut lexer: crate::lexer::Lexer = crate::lexer::Lexer {
        idx: 0,
        data: file_data_bytes,
        current: file_data_bytes[0],
        len: file_data_bytes.len(),
        line: 0,
        col: 0,
        info: &file_info,
    };

    let (_, tokens) = crate::lexer::generate_tokens(&mut lexer, &mut keywords);
    
    assert_eq!(tokens.len(), 12);
    assert_eq!(tokens.get(0).unwrap().tp, crate::lexer::TokenType::KEYWORD);
    assert_eq!(tokens.get(0).unwrap().data, "fn");
    assert_eq!(tokens.get(1).unwrap().tp, crate::lexer::TokenType::IDENTIFIER);
    assert_eq!(tokens.get(2).unwrap().tp, crate::lexer::TokenType::LPAREN);
    assert_eq!(tokens.get(3).unwrap().tp, crate::lexer::TokenType::RPAREN);
    assert_eq!(tokens.get(4).unwrap().tp, crate::lexer::TokenType::LCURLY);
    assert_eq!(tokens.get(5).unwrap().tp, crate::lexer::TokenType::IDENTIFIER);
    assert_eq!(tokens.get(6).unwrap().tp, crate::lexer::TokenType::NEWLINE);
    assert_eq!(tokens.get(7).unwrap().tp, crate::lexer::TokenType::I32);
    assert_eq!(tokens.get(8).unwrap().tp, crate::lexer::TokenType::NEWLINE);
    assert_eq!(tokens.get(9).unwrap().tp, crate::lexer::TokenType::STRING);
    assert_eq!(tokens.get(10).unwrap().tp, crate::lexer::TokenType::RCURLY);
    assert_eq!(tokens.get(11).unwrap().tp, crate::lexer::TokenType::EOF);
}

#[test]
fn test_parser() {
    let file_data: String = String::from("fn main(){Aa\n123\n\"🦅\"}");
    let filename: &String = &String::from("file.ke");

    let file_data_bytes: &[u8] = file_data.as_bytes();

    let file_info: crate::fileinfo::FileInfo = crate::fileinfo::FileInfo {
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
                                            ];

    let mut lexer: crate::lexer::Lexer = crate::lexer::Lexer {
        idx: 0,
        data: file_data_bytes,
        current: file_data_bytes[0],
        len: file_data_bytes.len(),
        line: 0,
        col: 0,
        info: &file_info,
    };

    let (_, tokens) = crate::lexer::generate_tokens(&mut lexer, &mut keywords);

    let mut parser: crate::parser::Parser = crate::parser::Parser {
        tokens: &tokens,
        idx: 1,
        current: tokens.first().unwrap().to_owned(),
        info: &file_info,
        allow_init: crate::parser::StructConstructionAllowance::new(),
    };

    let nodes: Vec<crate::parser::Node> = parser.genreate_ast();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes.first().unwrap().tp, crate::parser::NodeType::FUNC);
    assert_eq!(nodes.first().unwrap().data.func.as_ref().unwrap().blocks.get(0).unwrap().tp, crate::parser::NodeType::IDENTIFIER);
    assert_eq!(nodes.first().unwrap().data.func.as_ref().unwrap().blocks.get(1).unwrap().tp, crate::parser::NodeType::I32);
    assert_eq!(nodes.first().unwrap().data.func.as_ref().unwrap().blocks.get(2).unwrap().tp, crate::parser::NodeType::STRING);
}