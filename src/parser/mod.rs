pub mod nodes;
pub mod precedence;

use precedence::Precedence;

use crate::lexer;
use crate::errors::ErrorType;
use crate::lexer::TokenType;
use crate::codegen::types::{self, DataMutablility};

use self::nodes::NodeData;

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
    IDENTIFIER,
    FUNC,
    ASSIGN,
    CALL,
    RETURN,
    U32,
    I8,
    U8,
    I16,
    U16,
    I64,
    U64,
    I128,
    U128,
    TO,
    AS,
    F32,
    F64,
    UNARY,
    REF,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub isfn: bool,
    pub isref: bool,
    pub data: Option<String>,
    pub args: Option<Args>,
    pub mutability: types::DataMutablility,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Args {
    pub name: Vec<String>,
    pub args: Vec<Type>,
    pub rettp: Vec<Type>, //Only 1 element, Vec for indirection
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tp {
            NodeType::BINARY => write!(f, "{}", self.data.binary.as_ref().unwrap() ),
            NodeType::LET => write!(f, "{}", self.data.letn.as_ref().unwrap() ),
            NodeType::IDENTIFIER => write!(f, "{}", self.data.identifier.as_ref().unwrap() ),
            NodeType::FUNC => write!(f, "{}", self.data.func.as_ref().unwrap() ),
            NodeType::ASSIGN => write!(f, "{}", self.data.assign.as_ref().unwrap() ),
            NodeType::CALL => write!(f, "{}", self.data.call.as_ref().unwrap() ),
            NodeType::RETURN => write!(f, "{}", self.data.ret.as_ref().unwrap() ),
            NodeType::I8 |
            NodeType::U8 |
            NodeType::I16 |
            NodeType::U16 |
            NodeType::I32 |
            NodeType::U32 |
            NodeType::I64 |
            NodeType::U64 |
            NodeType::I128 |
            NodeType::F32 |
            NodeType::F64 |
            NodeType::U128 => write!(f, "{}", self.data.num.as_ref().unwrap() ),
            NodeType::TO |
            NodeType::AS => write!(f, "{}", self.data.to.as_ref().unwrap() ),
            NodeType::UNARY |
            NodeType::REF => write!(f, "{}", self.data.unary.as_ref().unwrap() ),
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

//Rules:
//Atomic: inplace
//Expressions + Keywords: leave off on next


impl<'life> Parser<'life> { 
    pub fn genreate_ast(&mut self)  -> Vec<Node> {        
        self.block()
    }

    fn raise_error(&mut self, error: &str, errtp: ErrorType) -> !{
        crate::errors::raise_error(error, errtp, &Position { line: self.current.line, startcol: self.current.startcol, endcol: self.current.endcol }, &self.info);
    }

    fn raise_error_pos(&mut self, error: &str, errtp: ErrorType, node: Node) -> !{
        crate::errors::raise_error(error, errtp, &node.pos, &self.info);
    }

    fn advance(&mut self) {
        let next = self.tokens.get(self.idx);
        self.idx+=1;

        match next {
            Some(v) => {
                self.current = v.to_owned();
            }
            None => {
                self.current = lexer::Token {
                    data: String::from("\0"),
                    tp: lexer::TokenType::EOF,
                    line: self.tokens.last().unwrap().line,
                    startcol: self.tokens.last().unwrap().startcol,
                    endcol: self.tokens.last().unwrap().endcol,
                };
            }
        }
    }

    fn backadvance(&mut self) {
        self.idx-=1;
        let next = self.tokens.get(self.idx-1);

        match next {
            Some(v) => {
                self.current = v.to_owned();
            }
            None => {
                self.current = lexer::Token {
                    data: String::from("\0"),
                    tp: lexer::TokenType::EOF,
                    line: self.tokens.last().unwrap().line,
                    startcol: self.tokens.last().unwrap().startcol,
                    endcol: self.tokens.last().unwrap().endcol,
                };
            }
        }
    }

    fn current_is_type(&mut self, tp: TokenType) -> bool {
        self.current.tp == tp
    }

    fn get_precedence(&self) -> Precedence {
        match self.current.tp {
            TokenType::EQUALS => {
                Precedence::Assign
            }
            TokenType::PLUS => {
                Precedence::Sum
            }
            TokenType::HYPHEN => {
                Precedence::Sum
            }
            TokenType::ASTERISK => {
                Precedence::Product
            }
            TokenType::FWSLASH => {
                Precedence::Product
            }
            TokenType::LPAREN => {
                Precedence::Call
            }
            TokenType::KEYWORD => {
                if self.current.data == "to" {
                    Precedence::To
                }
                else if self.current.data == "as" {
                    Precedence::To
                }
                else {
                    Precedence::Lowest
                }
            }
            _ => {
                Precedence::Lowest
            }
        }
    }

    fn create_node(&self, tp: NodeType, data: NodeData, pos: Position) -> Node {
        return Node {
            tp: tp,
            data: Box::new(data),
            pos,
        };
    }
    
    fn block(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        
        while !self.current_is_type(TokenType::EOF) && !self.current_is_type(TokenType::RCURLY) {
            nodes.push(self.statement());
            self.skip_newline();
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
                return self.expr(Precedence::Lowest);
            }
        }
        
    }

    fn is_atomic(&mut self) -> bool{
        match self.current.tp {
            TokenType::F32 |
            TokenType::F64 |
            TokenType::I32 |
            TokenType::U32 |
            TokenType::I8 |
            TokenType::U8 |
            TokenType::I16 |
            TokenType::U16 |
            TokenType::I64 |
            TokenType::U64 |
            TokenType::I128 |
            TokenType::U128 |
            TokenType::IDENTIFIER |
            TokenType::AMPERSAND |
            TokenType::PLUS |
            TokenType::HYPHEN |
            TokenType::LPAREN => return true,
            _ => return false,
        }
    }
    
    fn atom(&mut self) -> Option<Node> {
        match self.current.tp {
            TokenType::I32 => Some(self.generate_i32(self.current.data.clone())),
            TokenType::U32 => Some(self.generate_u32(self.current.data.clone())),
            TokenType::I8 => Some(self.generate_i8(self.current.data.clone())),
            TokenType::U8 => Some(self.generate_u8(self.current.data.clone())),
            TokenType::I16 => Some(self.generate_i16(self.current.data.clone())),
            TokenType::U16 => Some(self.generate_u16(self.current.data.clone())),
            TokenType::I64 => Some(self.generate_i64(self.current.data.clone())),
            TokenType::U64 => Some(self.generate_u64(self.current.data.clone())),
            TokenType::I128 => Some(self.generate_i128(self.current.data.clone())),
            TokenType::U128 => Some(self.generate_u128(self.current.data.clone())),
            TokenType::IDENTIFIER => Some(self.generate_identifier(self.current.data.clone())),
            TokenType::LPAREN => Some(self.generate_grouped()),
            TokenType::F32 => Some(self.generate_f32(self.current.data.clone())),
            TokenType::F64 => Some(self.generate_f64(self.current.data.clone())),
            TokenType::PLUS |
            TokenType::HYPHEN => Some(self.generate_unary()),
            TokenType::AMPERSAND => Some(self.generate_ref()),
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
        else if self.current.data == String::from("fn") {
            return self.parse_fn();
        }
        else if self.current.data == String::from("return") {
            return self.parse_return();
        }
        
        unreachable!();
    }
    
    fn expr(&mut self, prec: Precedence) -> Node {
        let mut left: Node;
        match self.atom() {
            None => self.raise_error("Invalid token.", ErrorType::InvalidTok),
            Some(val) => {left = val},
        }

        self.advance();
        while !self.current_is_type(TokenType::EOF) && (prec as u32) < (self.get_precedence() as u32){
            match self.current.tp {
                TokenType::PLUS |
                TokenType::HYPHEN |
                TokenType::ASTERISK |
                TokenType::FWSLASH => {
                    left = self.generate_binary(left, self.get_precedence());
                }

                TokenType::EQUALS => {
                    left = self.generate_assign(left);
                }

                TokenType::LPAREN => {
                    left = self.generate_call(left);
                }

                TokenType::KEYWORD => {
                    if self.current.data=="to" {
                        left = self.generate_to(left);
                    }
                    else if self.current.data=="as" {
                        left = self.generate_as(left);
                    }
                    else {
                        return left;
                    }
                }

                _ => {
                    return left;
                }
            }
        }
        if self.is_atomic() {
            self.raise_error("Unexpected token.", ErrorType::InvalidTok);
        }
        return left;
    }

    // Expressions
    
    fn generate_i32(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::I32, nodedat, pos);
    
        return n;
    }
    
    fn generate_binary(&mut self, left: Node, prec: Precedence) -> Node{
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
            _ => self.raise_error("Invalid token.", ErrorType::InvalidTok),
        };

        self.advance();
        
        let bin: nodes::BinaryNode = nodes::BinaryNode{
            left,
            op,
            right: self.expr(prec),
        };

        pos.endcol = bin.right.pos.endcol;

        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: Some(bin),
            num: None,
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };
    
        let n: Node = self.create_node(NodeType::BINARY, nodedat, pos);
    
        return n;
    }
    
    fn generate_identifier(&mut self, data: String) -> Node{
        let identi: nodes::IdentifierNode = nodes::IdentifierNode{
            name: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: None,
            identifier: Some(identi),
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::IDENTIFIER, nodedat, pos);
    
        return n;
    }

    fn generate_assign(&mut self, left: Node) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: left.pos.startcol,
            endcol: 0,
        };

        if left.tp != NodeType::IDENTIFIER {
            self.raise_error_pos("Expected identifier", ErrorType::InvalidTok, left);
        }

        self.advance();

        let assign: nodes::AssignNode = nodes::AssignNode{
            name: left.data.identifier.unwrap().name,
            expr: self.expr(Precedence::Lowest),
        };

        pos.endcol = assign.expr.pos.endcol;
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: None,
            identifier: None,
            func: None,
            assign: Some(assign),
            call: None,
            ret: None,
            to: None,
            unary: None,
        };
    
        let n: Node = self.create_node(NodeType::ASSIGN, nodedat, pos);
    
        return n;
    }

    fn generate_grouped(&mut self) -> Node {
        self.advance();
        let node: Node = self.expr(Precedence::Lowest);

        if !self.current_is_type(TokenType::RPAREN) {
            self.raise_error("Expected right parenthesis.", ErrorType::InvalidTok);
        }

        return node;
    }

    fn generate_call(&mut self, left: Node) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: left.pos.startcol,
            endcol: 0,
        };

        if left.tp != NodeType::IDENTIFIER {
            self.raise_error_pos("Expected identifier", ErrorType::InvalidTok, left);
        }

        let mut args: Vec<Node> = Vec::new();

        while !self.current_is_type(TokenType::RPAREN) {            
            self.advance();
            
            if self.current_is_type(TokenType::RPAREN) {
                break;
            }
            
            args.push(self.expr(Precedence::Lowest));

            if !self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::RPAREN) {
                self.raise_error("Expected comma.", ErrorType::InvalidTok);
            }
        }

        pos.endcol = self.current.endcol;

        self.advance();

        let call: nodes::CallNode = nodes::CallNode{
            name: left,
            args,
        };

    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: Some(call),
            ret: None,
            to: None,
            unary: None,
        };
    
        let n: Node = self.create_node(NodeType::CALL, nodedat, pos);
        return n;
    }
    
    fn generate_u32(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::U32, nodedat, pos);
    
        return n;
    }
    
    fn generate_i8(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::I8, nodedat, pos);
    
        return n;
    }
    
    fn generate_u8(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::U8, nodedat, pos);
    
        return n;
    }
    
    fn generate_i16(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::I16, nodedat, pos);
    
        return n;
    }
    
    fn generate_u16(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::U16, nodedat, pos);
    
        return n;
    }
    
    fn generate_i64(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::I64, nodedat, pos);
    
        return n;
    }
    
    fn generate_u64(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::U64, nodedat, pos);
    
        return n;
    }
    
    fn generate_i128(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::I128, nodedat, pos);
    
        return n;
    }
    
    fn generate_u128(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::U128, nodedat, pos);
    
        return n;
    }
    
    fn generate_to(&mut self, left: Node) -> Node{
        let mut pos = Position {
            line: left.pos.line,
            startcol: left.pos.startcol,
            endcol: 0,
        };

        self.advance();
        
        let res: (usize, Type) = self.parse_type(DataMutablility::Immutable, false);

        let to: nodes::ToNode = nodes::ToNode{
            left,
            tp: res.1,
        };

        pos.endcol = res.0;

        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: Some(to),
            unary: None,
        };
    
        let n: Node = self.create_node(NodeType::TO, nodedat, pos);
    
        return n;
    }
    
    fn generate_as(&mut self, left: Node) -> Node{
        let mut pos = Position {
            line: left.pos.line,
            startcol: left.pos.startcol,
            endcol: 0,
        };

        self.advance();
        
        let res: (usize, Type) = self.parse_type(DataMutablility::Immutable, false);

        let to: nodes::ToNode = nodes::ToNode{
            left,
            tp: res.1,
        };

        pos.endcol = res.0;

        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: Some(to),
            unary: None,
        };
    
        let n: Node = self.create_node(NodeType::AS, nodedat, pos);
    
        return n;
    }
    
    fn generate_f32(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::F32, nodedat, pos);
    
        return n;
    }
    
    fn generate_f64(&mut self, data: String) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone()
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: Some(int),
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::F64, nodedat, pos);
    
        return n;
    }
    
    fn generate_ref(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();

        let refn: nodes::UnaryNode = nodes::UnaryNode{
            op: nodes::UnaryOpType::REF,
            right: self.expr(Precedence::Lowest),
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: Some(refn),
        };

        self.backadvance();

        pos.endcol = nodedat.unary.as_ref().unwrap().right.pos.endcol;
    
        let n: Node = self.create_node(NodeType::REF, nodedat, pos);
    
        return n;
    }
    
    fn generate_unary(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };


        let op: nodes::UnaryOpType = match self.current.tp {
            TokenType::HYPHEN => nodes::UnaryOpType::NEG,
            TokenType::PLUS => nodes::UnaryOpType::POS,
            _ => self.raise_error("Invalid token.", ErrorType::InvalidTok),
        };

        self.advance();
        
        let un: nodes::UnaryNode = nodes::UnaryNode{
            op,
            right: self.expr(Precedence::Lowest),
        };

        pos.endcol = un.right.pos.endcol;

        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };
    
        let n: Node = self.create_node(NodeType::UNARY, nodedat, pos);
    
        return n;
    }

    //Keywords
    fn parse_let(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        let mut tp: Option<Type> = None;

        self.advance();
        
        let mut mutability: DataMutablility = DataMutablility::Immutable;
        if self.current_is_type(TokenType::KEYWORD) && self.current.data == "mut" {
            self.advance();
            mutability = DataMutablility::Mutable;
        }

        if !self.current_is_type(TokenType::IDENTIFIER) {
            self.raise_error("Expected identifier.", ErrorType::InvalidTok);
        }

        let name: String = self.current.data.clone();

        self.advance();
        
        if self.current_is_type(TokenType::COLON) {
            self.advance();
    
            tp=Some(self.parse_type(mutability, false).1);
        }


        if !self.current_is_type(TokenType::EQUALS) {
            self.raise_error("Expected equals.", ErrorType::InvalidTok);
        }
        
        self.advance();

        let expr: Node = self.expr(Precedence::Lowest);

        let letn: nodes::LetNode = nodes::LetNode{
            name,
            expr,
            mutability,
            tp,
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: Some(letn),
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

        pos.endcol = nodedat.letn.as_ref().unwrap().expr.pos.endcol;
    
        let n: Node = self.create_node(NodeType::LET, nodedat, pos);

        return n;        
    }

    fn parse_type(&mut self, mutability: DataMutablility, allow_ref: bool) -> (usize, Type){
        let mut isref: bool = false;

        if self.current_is_type(TokenType::AMPERSAND) && allow_ref {
            self.advance();
            isref = true;
        }

        if !self.current_is_type(TokenType::IDENTIFIER) {
            if !self.current_is_type(TokenType::KEYWORD) || (self.current_is_type(TokenType::IDENTIFIER) && self.current.data != "fn") {
                self.raise_error("Expected identifier.", ErrorType::InvalidTok);
            }
        }
        
        let tp: String = self.current.data.clone();
        if tp == "fn" {
            let mut args_: Args = Args {
                name: Vec::new(),
                args: Vec::new(),
                rettp: Vec::new(),
            };

            self.advance();
            
            if !self.current_is_type(TokenType::LPAREN) {
                self.raise_error("Expected left parenthesis.", ErrorType::InvalidTok);
            }

            self.advance();

            while !self.current_is_type(TokenType::RPAREN) && !self.current_is_type(TokenType::EOF) {
                let mut mutability: DataMutablility = DataMutablility::Immutable;
                if self.current_is_type(TokenType::KEYWORD) && self.current.data == "mut" {
                    self.advance();
                    mutability = DataMutablility::Mutable;
                }
                args_.args.push(self.parse_type(mutability, true).1);
            }
            
            if !self.current_is_type(TokenType::RPAREN) {
                self.raise_error("Expected right parenthesis.", ErrorType::InvalidTok);
            }

            let mut end: usize = self.current.endcol;

            self.advance();

            if self.current_is_type(TokenType::SMALLARROW) {
                self.advance();
                end = self.current.endcol;
                args_.rettp.push(self.parse_type(DataMutablility::Immutable, false).1);
            }
            else {
                args_.rettp.push(Type {
                    isfn: false,
                    isref: false,
                    data: Some(String::from("unit")),
                    args: None,
                    mutability: DataMutablility::Immutable,
                });
            }

            return (end, Type {
                isfn: true,
                isref: isref,
                data: None,
                args: Some(args_),
                mutability,
            });
        }
        else {
            let end: usize = self.current.endcol;
            self.advance();
            return (end, Type {
                isfn: false,
                isref: isref,
                data: Some(tp),
                args: None,
                mutability,
            });
        }
    }

    fn parse_fn(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();

        if !self.current_is_type(TokenType::IDENTIFIER) {
            self.raise_error("Expected identifier.", ErrorType::InvalidTok);
        }

        let name: String = self.current.data.clone();

        self.advance();

        if !self.current_is_type(TokenType::LPAREN) {
            self.raise_error("Expected left parenthesis.", ErrorType::InvalidTok);
        }
        
        self.advance();

        // Parse Arguments
        let mut args: Args = Args {
            name: Vec::new(),
            args: Vec::new(),
            rettp: Vec::new(),
        };
        while !self.current_is_type(TokenType::RPAREN) {
            let mut mutability: DataMutablility = DataMutablility::Immutable;
            if self.current_is_type(TokenType::KEYWORD) && self.current.data == "mut" {
                self.advance();
                mutability = DataMutablility::Mutable;
            }

            if !self.current_is_type(TokenType::IDENTIFIER) {
                self.raise_error("Expected identifier.", ErrorType::InvalidTok);
            }
    
            let name: String = self.current.data.clone();
            
            self.advance();

            if !self.current_is_type(TokenType::COLON) {
                self.raise_error("Expected colon.", ErrorType::InvalidTok);
            }

            self.advance();

            args.args.push(self.parse_type(mutability, true).1);
            if !self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::RPAREN) {
                self.raise_error("Expected comma.", ErrorType::InvalidTok);
            }
            
            args.name.push(name);

            if self.current_is_type(TokenType::RPAREN) {
                break;
            }

            self.advance();

        }

        //



        if !self.current_is_type(TokenType::RPAREN) {
            self.raise_error("Expected right parenthesis.", ErrorType::InvalidTok);
        }
        
        pos.endcol = self.current.endcol;

        self.advance();

        if self.current_is_type(TokenType::SMALLARROW) {
            self.advance();

            args.rettp.push(self.parse_type(DataMutablility::Immutable, false).1);
        }
        else {
            args.rettp.push(Type {
                isfn: false,
                isref: false,
                data: Some(String::from("unit")),
                args: None,
                mutability: DataMutablility::Immutable,
            });
        }


        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        self.skip_newline();

        let blocks: Vec<Node> = self.block();
        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        let func: nodes::FuncNode = nodes::FuncNode{
            name,
            blocks,
            args,
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn: None,
            identifier: None,
            func: Some(func),
            assign: None,
            call: None,
            ret: None,
            to: None,
            unary: None,
        };

    
        let n: Node = self.create_node(NodeType::FUNC, nodedat, pos);

        return n;        
    }

    fn parse_return(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();

        let expr: Node = self.expr(Precedence::Lowest);

        let retn: nodes::ReturnNode = nodes::ReturnNode{
            expr,
        };
    
        let nodedat: nodes::NodeData = nodes::NodeData {
            binary: None,
            num: None,
            letn:  None,
            identifier: None,
            func: None,
            assign: None,
            call: None,
            ret: Some(retn),
            to: None,
            unary: None,
        };

        pos.endcol = nodedat.ret.as_ref().unwrap().expr.pos.endcol;
    
        let n: Node = self.create_node(NodeType::RETURN, nodedat, pos);

        return n;        
    }

}