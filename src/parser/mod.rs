mod nodes;
use crate::lexer;
use crate::errors::ErrorType;
use crate::lexer::TokenType;

pub struct Parser<'life> {
    pub tokens: &'life Vec<lexer::Token>,
    pub idx: usize,
    pub current: lexer::Token,
    pub info: &'life crate::fileinfo::FileInfo<'life>,
}

#[derive(Clone, Copy)]
pub enum NodeType {
    BINARY,
    INT,
}

#[derive(Clone)]
pub struct Node{
    tp: NodeType,
    data: Box<nodes::NodeData>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tp {
            NodeType::BINARY => write!(f, "{}", self.data.binary.as_ref().unwrap() ),
            NodeType::INT => write!(f, "{}", self.data.int.as_ref().unwrap() ),
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
    pub fn genreate_ast(&mut self) {        
        self.block();
    }

    fn raise_error(&mut self, error: &str, errtp: ErrorType) -> !{
        crate::errors::raise_error(error, errtp, self.current.line, self.current.startcol, self.current.endcol, &self.info);
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
    
    fn block(&mut self) {
        let mut nodes: Vec<Node> = Vec::new();
        
        while !self.current_is_type(TokenType::EOF) {
            nodes.push(self.statement());
            self.advance();
        }

        print_nodes(&nodes);
    }
    
    fn statement(&mut self) -> Node{
        //Later, handle keywords here
        return self.expr();
    }
    
    fn atom(&mut self) -> Option<Node> {
        match self.current.tp {
            TokenType::INT => Some(self.generate_int(self.current.data.clone())),
            _ => None,
        }
    }

    fn skip_newline(&mut self) {
        while self.current_is_type(TokenType::NEWLINE) {
            self.advance();
        }
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
                TokenType::INT |
                TokenType::PLUS => {
                    left = self.generate_binary(left);
                }
                _ => {}
            }
        }

        return left;
    }
    
    fn generate_int(&mut self, data: String) -> Node{
        let int: nodes::IntNode = nodes::IntNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            int: Some(int),
        };
    
        let n: Node = Node {
            tp: NodeType::INT,
            data: Box::new(nodedat),
        };
    
        return n;
    }
    
    fn generate_binary(&mut self, left: Node) -> Node{
        let op: nodes::BinaryOpType = match self.current.tp {
            TokenType::PLUS => nodes::BinaryOpType::ADD,
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
        };
    
        let n: Node = Node {
            tp: NodeType::BINARY,
            data: Box::new(nodedat),
        };
    
        return n;
    }

}