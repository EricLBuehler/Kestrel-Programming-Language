pub mod nodes;
use crate::lexer;
use crate::errors::ErrorType;
use crate::lexer::TokenType;

pub struct Parser<'life> {
    pub tokens: &'life Vec<lexer::Token>,
    pub idx: usize,
    pub current: lexer::Token,
    pub info: &'life crate::fileinfo::FileInfo<'life>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum NodeType {
    BINARY,
    I32,
    LET,
}

#[derive(Clone)]
pub struct Node{
    pub tp: NodeType,
    pub data: Box<nodes::NodeData>,
    pub pos: Position,
}

#[derive(Clone)]
pub struct Position{
    pub line: usize,
    pub startcol: usize,
    pub endcol: usize,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tp {
            NodeType::BINARY => write!(f, "{}", self.data.binary.as_ref().unwrap() ),
            NodeType::I32 => write!(f, "{}", self.data.int.as_ref().unwrap() ),
            NodeType::LET => write!(f, "{}", self.data.letn.as_ref().unwrap() ),
        }
    }    
}

#[allow(dead_code)]
pub fn print_nodes(nodes: &Vec<Node>) {
    println!("\n\nGenerated AST:\n========================");
    println!("Abstract Syntax Tree ({} head nodes)", nodes.len());
    println!("------------------------");
    for idx in 0..nodes.len() {
        println!("{} | {}", idx+1, nodes.get(idx).unwrap());
    }
    println!("========================");
}

// ---------------------------------------------------------------------------------------------------------------
//                                      Parser internal functions
// ---------------------------------------------------------------------------------------------------------------


impl<'life> Parser<'life> { 
    pub fn genreate_ast(&mut self)  -> Vec<Node> {        
        self.block()
    }

    fn raise_error(&mut self, error: &str, errtp: ErrorType) -> !{
        crate::errors::raise_error(error, errtp, &Position { line: self.current.line, startcol: self.current.startcol, endcol: self.current.endcol }, &self.info);
    }

    fn advance(&mut self) {
        self.idx+=1;
        let next: Option<&lexer::Token> = self.tokens.get(self.idx-1);

        match next {
            Some(v) => {
                self.current = v.to_owned();
            }
            None => {
                self.current = lexer::Token {
                    data: String::from("\0"),
                    tp: lexer::TokenType::EOF,
                    line: 0,
                    startcol: 0,
                    endcol: 0,
                };
            }
        }        
    }
    
    fn current_is_type(&mut self, tp: TokenType) -> bool {
        self.current.tp == tp
    }
    
    fn block(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        
        while !self.current_is_type(TokenType::EOF) {
            nodes.push(self.statement());
            self.advance();
        }

        print_nodes(&nodes);
        return nodes;
    }
    
    fn statement(&mut self) -> Node{
        match self.current.tp {
            lexer::TokenType::KEYWORD => {
                return self.keyword();
            }
            _ => {
                return self.expr();
            }
        }
        
    }
    
    fn atom(&mut self) -> Option<Node> {
        match self.current.tp {
            TokenType::I32 => Some(self.generate_int(self.current.data.clone())),
            _ => None,
        }
    }

    fn skip_newline(&mut self) {
        while self.current_is_type(TokenType::NEWLINE) {
            self.advance();
        }
    }

    fn keyword(&mut self) -> Node {
        if self.current.data == String::from("let") {
            return self.parse_let();
        }
        
        unreachable!();
    }
    
    fn expr(&mut self) -> Node {
        let mut left: Node;
        match self.atom() {
            None => self.raise_error("invalid token.", ErrorType::InvalidTok),
            Some(val) => {left = val},
        }

        while !self.current_is_type(TokenType::NEWLINE) && !self.current_is_type(TokenType::EOF) {
            self.skip_newline();
            self.advance();

            match self.current.tp {
                TokenType::PLUS |
                TokenType::HYPHEN |
                TokenType::ASTERISK |
                TokenType::FWSLASH => {
                    left = self.generate_binary(left);
                }
                _ => {}
            }
        }

        return left;
    }

    // Expressions
    
    fn generate_int(&mut self, data: String) -> Node{
        let int: nodes::I32Node = nodes::I32Node{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            int: Some(int),
            letn: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = Node {
            tp: NodeType::I32,
            data: Box::new(nodedat),
            pos,
        };
    
        return n;
    }
    
    fn generate_binary(&mut self, left: Node) -> Node{
        let mut pos = Position {
            line: left.pos.line,
            startcol: left.pos.startcol,
            endcol: 0,
        };


        let op: nodes::BinaryOpType = match self.current.tp {
            TokenType::PLUS => nodes::BinaryOpType::ADD,
            TokenType::HYPHEN => nodes::BinaryOpType::SUB,
            TokenType::ASTERISK => nodes::BinaryOpType::MUL,
            TokenType::FWSLASH => nodes::BinaryOpType::DIV,
            _ => nodes::BinaryOpType::DEFAULT,
        };

        self.advance();
        
        let bin: nodes::BinaryNode = nodes::BinaryNode{
            left,
            op,
            right: self.expr(),
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: Some(bin),
            int: None,
            letn: None,
        };

        pos.endcol = self.current.endcol;
    
        let n: Node = Node {
            tp: NodeType::BINARY,
            data: Box::new(nodedat),
            pos,
        };
    
        return n;
    }

    //Keywords
    fn parse_let(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();

        if !self.current_is_type(TokenType::IDENTIFIER) {
            self.raise_error("expected identifier.", ErrorType::InvalidTok);
        }

        let name: String = self.current.data.clone();

        self.advance();

        if !self.current_is_type(TokenType::EQUALS) {
            self.raise_error("expected equals.", ErrorType::InvalidTok);
        }
        
        self.advance();

        let expr: Node = self.expr();

        let letn: nodes::LetNode = nodes::LetNode{
            name,
            expr,
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            int: None,
            letn: Some(letn),
        };

        pos.endcol = self.current.endcol;
    
        let n: Node = Node {
            tp: NodeType::LET,
            data: Box::new(nodedat),
            pos,
        };

        return n;        
    }

}