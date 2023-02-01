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

#[derive(Clone, Copy, PartialEq, Debug)]
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
    AS,
    F32,
    F64,
    UNARY,
    REF,
    STRUCT,
    INITSTRUCT,
    ATTR,
    ATTRASSIGN,
    STRING,
    CHAR,
    ARRAY,
    IMPL,
    NAMESPACE,
    IF,
    LOOP,
    BREAK,
    CONTINUE,
    WHILE,
    ENUM,
    TRAIT,
    VOID,
    IS,
    MATCH,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node{
    pub tp: NodeType,
    pub data: Box<nodes::NodeData>,
    pub pos: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position{
    pub line: usize,
    pub startcol: usize,
    pub endcol: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub isfn: bool,
    pub isarr: bool,
    pub isdyn: bool,
    pub arrtp: Option<Box<Type>>,
    pub arrlen: Option<Vec<String>>,
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
            NodeType::U128 |
            NodeType::CHAR => write!(f, "{}", self.data.num.as_ref().unwrap() ),
            NodeType::AS => write!(f, "{}", self.data.to.as_ref().unwrap() ),
            NodeType::UNARY |
            NodeType::REF => write!(f, "{}", self.data.unary.as_ref().unwrap() ),
            NodeType::STRUCT => write!(f, "{}", self.data.st.as_ref().unwrap() ),
            NodeType::INITSTRUCT => write!(f, "{}", self.data.initst.as_ref().unwrap() ),
            NodeType::ATTR |
            NodeType::NAMESPACE => write!(f, "{}", self.data.attr.as_ref().unwrap() ),
            NodeType::ATTRASSIGN => write!(f, "{}", self.data.attrassign.as_ref().unwrap() ),
            NodeType::STRING => write!(f, "{}", self.data.str.as_ref().unwrap() ),
            NodeType::ARRAY => write!(f, "{}", self.data.arr.as_ref().unwrap() ),
            NodeType::IMPL => write!(f, "{}", self.data.impln.as_ref().unwrap() ),
            NodeType::IF => write!(f, "{}", self.data.ifn.as_ref().unwrap() ),
            NodeType::LOOP |
            NodeType::WHILE => write!(f, "{}", self.data.loopn.as_ref().unwrap() ),
            NodeType::BREAK |
            NodeType::CONTINUE => Ok(()),
            NodeType::ENUM => write!(f, "{}", self.data.enumn.as_ref().unwrap() ),
            NodeType::TRAIT => write!(f, "{}", self.data.traitn.as_ref().unwrap() ),
            NodeType::VOID => write!(f, "void"),
            NodeType::IS => write!(f, "{}", self.data.is.as_ref().unwrap() ),
            NodeType::MATCH => write!(f, "{}", self.data.matchn.as_ref().unwrap() ),
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

    fn next_is_type(&mut self, tp: TokenType) -> bool {
        self.advance();
        let res: bool = self.current.tp == tp;
        self.backadvance();
        return res;
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
                else if self.current.data == "is" {
                    Precedence::Comparison
                }
                else {
                    Precedence::Lowest
                }
            }
            TokenType::GT |
            TokenType::GTE |
            TokenType::LT |
            TokenType::LTE  => {
                Precedence::Comparison
            }
            TokenType::EQ |
            TokenType::NE => {
                Precedence::Equals
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
        self.skip_newline();
        
        while !self.current_is_type(TokenType::EOF) && !self.current_is_type(TokenType::RCURLY) {
            nodes.push(self.statement());
            self.skip_newline();
        }

        //print_nodes(&nodes);
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
            TokenType::LPAREN |
            TokenType::STRING => return true,
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
            TokenType::STRING => Some(self.generate_str()),
            TokenType::CHAR => Some(self.generate_char(self.current.data.clone())),
            TokenType::LSQUARE => Some(self.generate_array()),
            TokenType::KEYWORD => if self.current.data == "void" { Some(self.generate_void()) } else if self.current.data == "if" { let v: Option<Node> = Some(self.parse_if(true)); self.backadvance(); v }else { None },
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
        else if self.current.data == String::from("struct") {
            return self.parse_struct();
        }
        else if self.current.data == String::from("impl") {
            return self.parse_impl();
        }
        else if self.current.data == String::from("if") {
            return self.parse_if(false);
        }
        else if self.current.data == String::from("loop") {
            return self.parse_loop();
        }
        else if self.current.data == String::from("break") {
            return self.parse_break();
        }
        else if self.current.data == String::from("continue") {
            return self.parse_continue();
        }
        else if self.current.data == String::from("while") {
            return self.parse_while();
        }
        else if self.current.data == String::from("enum") {
            return self.parse_enum();
        }
        else if self.current.data == String::from("trait") {
            return self.parse_trait();
        }
        else if self.current.data == String::from("match") {
            return self.parse_match();
        }
        
        self.raise_error("Invalid keyword.", ErrorType::InvalidTok);
    }
    
    fn expr(&mut self, prec: Precedence) -> Node {
        if self.current_is_type(TokenType::EMOJIERR) {
            self.raise_error("Identifiers cannot contain emojis.", ErrorType::InvalidTok);
        }
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
                TokenType::FWSLASH |
                TokenType::GT |
                TokenType::GTE |
                TokenType::LT |
                TokenType::LTE |
                TokenType::EQ |
                TokenType::NE => {
                    left = self.generate_binary(left, self.get_precedence());
                }

                TokenType::EQUALS => {
                    left = self.generate_assign(left);
                }

                TokenType::LPAREN => {
                    left = self.generate_call(left);
                }

                TokenType::KEYWORD => {
                    if self.current.data=="as" {
                        left = self.generate_as(left);
                    }
                    else if self.current.data=="is" {
                        left = self.generate_is(left);
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            TokenType::GT => nodes::BinaryOpType::GT,
            TokenType::GTE => nodes::BinaryOpType::GTE,
            TokenType::LT => nodes::BinaryOpType::LT,
            TokenType::LTE => nodes::BinaryOpType::LTE,
            TokenType::EQ => nodes::BinaryOpType::EQ,
            TokenType::NE => nodes::BinaryOpType::NE,
            _ => self.raise_error("Invalid token.", ErrorType::InvalidTok),
        };

        self.advance();

        let mut isassign: bool = false;

        if self.current_is_type(TokenType::EQUALS) {
            isassign = true;
            if left.tp != NodeType::IDENTIFIER {
                self.raise_error("Expected identifier.", ErrorType::InvalidTok);
            }
            self.advance();
        }
        
        let bin: nodes::BinaryNode = nodes::BinaryNode{
            left,
            op,
            right: self.expr(prec),
            isassign,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let mut n: Node = self.create_node(NodeType::IDENTIFIER, nodedat, pos.clone());

        if self.next_is_type(TokenType::LCURLY) {
            let name: String = self.current.data.clone();
            self.advance();
            self.advance();

            let mut members: std::collections::HashMap<String, Node> = std::collections::HashMap::new();
            let mut members_vec: Vec<String> = Vec::new();
            
            self.skip_newline();
    
            while !self.current_is_type(TokenType::RCURLY) && !self.current_is_type(TokenType::EOF) {
                if self.current_is_type(TokenType::RCURLY) {
                    break;
                }
    
                if !self.current_is_type(TokenType::IDENTIFIER) {
                    self.raise_error("Expected identifier.", ErrorType::InvalidTok);
                }
        
                let name: String = self.current.data.clone();
        
                self.advance();
    
                if !self.current_is_type(TokenType::EQUALS) {
                    self.raise_error("Expected equals.", ErrorType::InvalidTok);
                }
    
                self.advance();
                
                members.insert(name.clone(), self.expr(Precedence::Lowest));
                members_vec.push(name);

                self.skip_newline();

                if !self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::RCURLY) {
                    self.raise_error("Expected comma.", ErrorType::InvalidTok);
                }

                if self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::RCURLY) {
                    self.advance();
                }

                self.skip_newline();
            }
            self.skip_newline();
            
            if !self.current_is_type(TokenType::RCURLY) {
                self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
            }


            let initst: nodes::StructInitNode = nodes::StructInitNode{
                name,
                members,
                members_vec,
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
                unary: None,
                st: None,
                initst: Some(initst),
                attr: None,
                attrassign: None,
                str: None,
                arr: None,
                impln: None,
                ifn: None,
                loopn: None,
                enumn: None,
                traitn: None,
                is: None,
                matchn: None,
            };
        
            n = self.create_node(NodeType::INITSTRUCT, nodedat, pos);
        }
        else if self.next_is_type(TokenType::DOT) {
            let mut pos = Position {
                line: self.current.line,
                startcol: self.current.startcol,
                endcol: 0,
            };
            self.advance();
            self.advance();
            let attr: String = self.current.data.clone();

            if self.next_is_type(TokenType::EQUALS) {
                self.advance();
                self.advance();
                let expr: Node = self.expr(Precedence::Lowest);
                self.backadvance();
                pos.endcol = self.current.endcol;

                let attr: nodes::AttrAssignNode = nodes::AttrAssignNode{
                    name: n,
                    attr,
                    expr,
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
                    unary: None,
                    st: None,
                    initst: None,
                    attr: None,
                    attrassign: Some(attr),
                    str: None,
                    arr: None,
                    impln: None,
                    ifn: None,
                    loopn: None,
                    enumn: None,
                    traitn: None,
                    is: None,
                    matchn: None,
                };
            
                n = self.create_node(NodeType::ATTRASSIGN, nodedat, pos.clone());
            }
            else {            
                pos.endcol = self.current.endcol;

                let attr: nodes::AttrNode = nodes::AttrNode{
                    name: n,
                    attr,
                    expr: None,
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
                    unary: None,
                    st: None,
                    initst: None,
                    attr: Some(attr),
                    attrassign: None,
                    str: None,
                    arr: None,
                    impln: None,
                    ifn: None,
                    loopn: None,
                    enumn: None,
                    traitn: None,
                    is: None,
                    matchn: None,
                };
            
                n = self.create_node(NodeType::ATTR, nodedat, pos.clone());
            }
        }
        else if self.next_is_type(TokenType::DOUBLECOLON) {
            let mut pos = Position {
                line: self.current.line,
                startcol: self.current.startcol,
                endcol: 0,
            };
            self.advance();
            self.advance();
            let attr: String = self.current.data.clone();
            let expr: Option<Node>;
            
            if self.next_is_type(TokenType::LT) {
                self.advance();
                self.advance();

                expr = Some(self.expr(Precedence::Comparison));
                
                if !self.current_is_type(TokenType::GT) {
                    self.raise_error("Expected right angle bracket.", ErrorType::InvalidTok);
                }
            }
            else {
                expr = None;
            }

            pos.endcol = self.current.endcol;

            let attr: nodes::AttrNode = nodes::AttrNode{
                name: n,
                attr,
                expr,
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
                unary: None,
                st: None,
                initst: None,
                attr: Some(attr),
                attrassign: None,
                str: None,
                arr: None,
                impln: None,
                ifn: None,
                loopn: None,
                enumn: None,
                traitn: None,
                is: None,
                matchn: None,
            };
        
            n = self.create_node(NodeType::NAMESPACE, nodedat, pos.clone());
        }
    
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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

        if  left.tp != NodeType::IDENTIFIER &&
            left.tp != NodeType::ATTR &&
            left.tp != NodeType::NAMESPACE {
            self.raise_error_pos("Expected name", ErrorType::InvalidTok, left);
        }

        let mut args: Vec<Node> = Vec::new();

        while !self.current_is_type(TokenType::RPAREN) && !self.current_is_type(TokenType::EOF) {            
            self.advance();
            
            self.skip_newline();
            
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::U128, nodedat, pos);
    
        return n;
    }
    
    fn generate_as(&mut self, left: Node) -> Node{
        let mut pos = Position {
            line: left.pos.line,
            startcol: left.pos.startcol,
            endcol: 0,
        };

        self.advance();
        
        let res: (usize, Type) = self.parse_type(DataMutablility::Immutable);

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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };
    
        let n: Node = self.create_node(NodeType::UNARY, nodedat, pos);
    
        return n;
    }
    
    fn generate_str(&mut self) -> Node{
        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };

        let str: nodes::StringNode = nodes::StringNode{
            data: self.current.data.clone(),
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: Some(str),
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };
    
        let n: Node = self.create_node(NodeType::STRING, nodedat, pos);
    
        return n;
    }
    
    fn generate_char(&mut self, data: String) -> Node{
        if data.len() == 0 {
            self.raise_error("Char literal cannot be empty.", ErrorType::EmptyCharLiteral);
        }

        if data.len() > 4 {
            self.raise_error("Invalid multibyte sequence (>4 bytes).", ErrorType::UnexpectedMultibyte);
        }

        let int: nodes::NumNode = nodes::NumNode{
            left: data.clone(),
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::CHAR, nodedat, pos);
    
        return n;
    }
    
    fn generate_array(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        let mut elements: Vec<Node> = Vec::new();

        self.advance();

        while !self.current_is_type(TokenType::RSQUARE) && !self.current_is_type(TokenType::EOF) {
            elements.push(self.expr(Precedence::Lowest));
            self.skip_newline();
            if !self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::RSQUARE) {
                self.raise_error("Expected comma.", ErrorType::InvalidTok);
            }
            if self.current_is_type(TokenType::RSQUARE) {
                break;
            }
            self.advance();
            self.skip_newline();
        }
        if !self.current_is_type(TokenType::RSQUARE) {
            self.raise_error("Expected right square bracket.", ErrorType::InvalidTok);
        }
        
        pos.endcol = self.current.endcol;

        let arr: nodes::ArrayNode = nodes::ArrayNode{
            elements,
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: Some(arr),
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };
    
        let n: Node = self.create_node(NodeType::ARRAY, nodedat, pos);
    
        return n;
    }

    fn generate_void(&mut self) -> Node {
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

    
        let n: Node = self.create_node(NodeType::VOID, nodedat, Position {startcol: self.current.startcol, endcol: self.current.endcol, line: self.current.line } );

        return n;   
    }
    
    fn generate_is(&mut self, left: Node) -> Node{
        let mut pos = Position {
            line: left.pos.line,
            startcol: left.pos.startcol,
            endcol: 0,
        };

        self.advance();
        
        let expr: Node = self.expr(Precedence::Lowest);

        if expr.tp != NodeType::NAMESPACE {
            self.raise_error("Expected namespace or enum attribute access.", ErrorType::InvalidTok);
        }

        pos.endcol = expr.pos.endcol;

        let is: nodes::IsNode = nodes::IsNode{
            left,
            variant: expr,
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: Some(is),
            matchn: None,
        };
    
        let n: Node = self.create_node(NodeType::IS, nodedat, pos);
    
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
    
            tp=Some(self.parse_type(mutability).1);
        }

        let mut expr: Option<Node> = None;

        if self.current_is_type(TokenType::EQUALS) {
            self.advance();
    
            expr = Some(self.expr(Precedence::Lowest));
        }
        else {
            pos.endcol = self.current.endcol;
            self.advance()
        }
        

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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };
        
        if nodedat.letn.as_ref().unwrap().expr.is_some() {
            pos.endcol = nodedat.letn.as_ref().unwrap().expr.as_ref().unwrap().pos.endcol;
        }
    
        let n: Node = self.create_node(NodeType::LET, nodedat, pos);

        return n;        
    }

    fn parse_type(&mut self, mutability: DataMutablility) -> (usize, Type){
        if self.current_is_type(TokenType::KEYWORD) && self.current.data == "dyn" {
            self.advance();
            if !self.current_is_type(TokenType::IDENTIFIER) {
                self.raise_error("Expected identifier.", ErrorType::InvalidTok);
            }
            let end: usize = self.current.endcol;
            let name: String = self.current.data.to_owned();
            self.advance();
            return (end, Type {
                isfn: false,
                isarr: false,
                isdyn: true,
                arrtp: None,
                arrlen: None,
                data: Some(name),
                args: None,
                mutability,
            });
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
                args_.args.push(self.parse_type(mutability).1);
            }
            
            if !self.current_is_type(TokenType::RPAREN) {
                self.raise_error("Expected right parenthesis.", ErrorType::InvalidTok);
            }

            let mut end: usize = self.current.endcol;

            self.advance();

            if self.current_is_type(TokenType::SMALLARROW) {
                self.advance();
                end = self.current.endcol;
                args_.rettp.push(self.parse_type(DataMutablility::Immutable).1);
            }
            else {
                args_.rettp.push(Type {
                    isfn: false,
                    isarr: false,
                    isdyn: false,
                    arrtp: None,
                    arrlen: None,
                    data: Some(String::from("void")),
                    args: None,
                    mutability: DataMutablility::Immutable,
                });
            }

            return (end, Type {
                isfn: true,
                isarr: false,
                isdyn: false,
                arrtp: None,
                arrlen: None,
                data: None,
                args: Some(args_),
                mutability,
            });
        }
        else if self.next_is_type(TokenType::LSQUARE) {
            let basetp: Type = Type {
                isfn: false,
                isarr: false,
                isdyn: false,
                arrtp: None,
                arrlen: None,
                data: Some(tp),
                args: None,
                mutability,
            };
            
            self.advance();

            let mut len: Vec<String> = Vec::new();

            while self.current_is_type(TokenType::LSQUARE) {
                self.advance();
                if  !(self.current_is_type(TokenType::I32) &&
                    self.current.data.chars().nth(0).unwrap() != '-') {
                    self.raise_error("Expected positive integer.", ErrorType::InvalidTok);
                }
                len.push(self.current.data.clone());
                self.advance();
                if !self.current_is_type(TokenType::RSQUARE) {
                    self.raise_error("Expected right square bracket.", ErrorType::InvalidTok);
                }
                self.advance();
            }

            let end: usize = self.current.endcol;

            return (end, Type {
                isfn: false,
                isarr: true,
                isdyn: false,
                arrtp: Some(Box::new(basetp)),
                arrlen: Some(len),
                data: None,
                args: None,
                mutability,
            });
        }
        else {
            let end: usize = self.current.endcol;
            self.advance();
            return (end, Type {
                isfn: false,
                isarr: false,
                isdyn: false,
                arrtp: None,
                arrlen: None,
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
        let mut methodname: Option<String> = None;
        let mut namespacename: Option<String> = None;
        let mut template_types: Vec<String> = Vec::new();

        self.advance();

        if self.current_is_type(TokenType::DOT) {
            self.advance();
            if !self.current_is_type(TokenType::IDENTIFIER) {
                self.raise_error("Expected identifier.", ErrorType::InvalidTok);
            }
            methodname = Some(self.current.data.clone());
            self.advance();
        }

        if self.current_is_type(TokenType::DOUBLECOLON) {
            self.advance();
            if !self.current_is_type(TokenType::IDENTIFIER) {
                self.raise_error("Expected identifier.", ErrorType::InvalidTok);
            }
            namespacename = Some(self.current.data.clone());
            self.advance();
        }        

        if self.current_is_type(TokenType::LT) {
            self.advance();
            while self.current_is_type(TokenType::IDENTIFIER) {
                template_types.push(self.current.data.clone());

                self.advance();

                if !self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::GT) {
                    self.raise_error("Expected comma.", ErrorType::InvalidTok);
                }
                self.advance();

                if self.current_is_type(TokenType::GT) {
                    self.advance();
                    break;
                }
            }
        }

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
        while !self.current_is_type(TokenType::RPAREN) && !self.current_is_type(TokenType::EOF) {
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

            args.args.push(self.parse_type(mutability).1);
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

            args.rettp.push(self.parse_type(DataMutablility::Immutable).1);

            self.backadvance();

            pos.endcol = self.current.endcol;
            
            self.advance();
        }
        else {
            args.rettp.push(Type {
                isfn: false,
                isarr: false,
                isdyn: false,
                arrtp: None,
                arrlen: None,
                data: Some(String::from("void")),
                args: None,
                mutability: DataMutablility::Immutable,
            });
        }

        self.skip_newline();

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
            methodname,
            namespacename,
            template_types,
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
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

        let expr: Option<Node>;

        if self.current_is_type(TokenType::NEWLINE) {
            expr = None;
        }
        else {
            expr = Some(self.expr(Precedence::Lowest));
        }

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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

        if nodedat.ret.as_ref().unwrap().expr.is_some() {
            pos.endcol = nodedat.ret.as_ref().unwrap().expr.as_ref().unwrap().pos.endcol;
        }
        else {
            pos.endcol = self.current.endcol;
        }
    
        let n: Node = self.create_node(NodeType::RETURN, nodedat, pos);

        return n;        
    }

    fn parse_struct(&mut self) -> Node{
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

        pos.endcol = self.current.endcol;

        self.advance();

        
        self.skip_newline();

        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        let mut members: std::collections::HashMap<String, Type> = std::collections::HashMap::new();
        let mut names: Vec<String> = Vec::new();
        
        self.skip_newline();

        while !self.current_is_type(TokenType::RCURLY) && !self.current_is_type(TokenType::EOF) {
            if self.current_is_type(TokenType::RCURLY) {
                break;
            }

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

            let (_, tp) = self.parse_type(mutability);
            
            members.insert(name.clone(), tp);
            names.push(name);

            self.skip_newline();

            if !self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::RCURLY) {
                self.raise_error("Expected comma.", ErrorType::InvalidTok);
            }

            if self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::RCURLY) {
                self.advance();
            }

            self.skip_newline();
        }
        self.skip_newline();
        
        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        let st: nodes::StructNode = nodes::StructNode{
            name,
            names,
            members,
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
            unary: None,
            st: Some(st),
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

    
        let n: Node = self.create_node(NodeType::STRUCT, nodedat, pos);

        return n;        
    }

    fn parse_impl(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();

        if !self.current_is_type(TokenType::IDENTIFIER) {
            self.raise_error("Expected identifier.", ErrorType::InvalidTok);
        }

        let traitnm: String = self.current.data.clone();

        self.advance();

        if !self.current_is_type(TokenType::KEYWORD) || (self.current_is_type(TokenType::KEYWORD) && self.current.data != "for") {
            self.raise_error("Expected identifier.", ErrorType::InvalidTok);
        }

        self.advance();

        if !self.current_is_type(TokenType::IDENTIFIER) {
            self.raise_error("Expected identifier.", ErrorType::InvalidTok);
        }

        let structnm: String = self.current.data.clone();

        self.advance();

        pos.endcol = self.current.endcol;

        self.skip_newline();
    
        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        self.skip_newline();

        let mut functions: Vec<Node> = Vec::new();

        while !self.current_is_type(TokenType::RCURLY) && !self.current_is_type(TokenType::EOF) {
            if !self.current_is_type(TokenType::KEYWORD) && self.current.data == "fn" {
                self.raise_error("Expected fn.", ErrorType::InvalidStatement);
            }

            let stmt: Node = self.statement();

            self.skip_newline();

            functions.push(stmt);            
        
            if self.current_is_type(TokenType::RCURLY) {
                break;
            }
        }

        self.skip_newline();
        
        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        let impln: nodes::ImplNode = nodes::ImplNode{
            functions,
            traitnm,
            structnm,
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: Some(impln),
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

    
        let n: Node = self.create_node(NodeType::IMPL, nodedat, pos);

        return n;        
    }

    fn parse_if(&mut self, inexpr: bool) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();

        let mut ifs: Vec<(Node, Vec<Node>)> = Vec::new();
        let mut else_opt: Option<Vec<Node>> = None;


        let expr: Node = self.expr(Precedence::Lowest);

        pos.endcol = expr.pos.endcol;

        self.skip_newline();
    
        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        self.skip_newline();

        let body: Vec<Node> = self.block();

        self.skip_newline();
        
        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        self.skip_newline();

        ifs.push((expr, body));

        while self.current_is_type(TokenType::KEYWORD) && self.current.data == "elif" {
            self.advance(); 

            let expr: Node = self.expr(Precedence::Lowest);    
        
            self.skip_newline();

            if !self.current_is_type(TokenType::LCURLY) {
                self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
            }
            
            self.advance();
    
            self.skip_newline();
    
            let body: Vec<Node> = self.block();
    
            self.skip_newline();
            
            if !self.current_is_type(TokenType::RCURLY) {
                self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
            }
    
            self.advance();
    
            self.skip_newline();
    
            ifs.push((expr, body));
        }

        if self.current_is_type(TokenType::KEYWORD) && self.current.data == "else" {
            self.advance(); 
        
            self.skip_newline();

            if !self.current_is_type(TokenType::LCURLY) {
                self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
            }
            
            self.advance();
    
            self.skip_newline();
    
            let body: Vec<Node> = self.block();
    
            self.skip_newline();
            
            if !self.current_is_type(TokenType::RCURLY) {
                self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
            }
    
            self.advance();
    
            self.skip_newline();
    
            else_opt = Some(body);
        }

        let ifn: nodes::IfNode = nodes::IfNode{
            ifs,
            else_opt,
            inexpr,
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: Some(ifn),
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

    
        let n: Node = self.create_node(NodeType::IF, nodedat, pos);

        return n;        
    }

    fn parse_loop(&mut self) -> Node{
        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };

        self.advance();
    
        self.skip_newline();

        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        self.skip_newline();
        
        let block: Vec<Node> = self.block();

        self.skip_newline();
        
        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        let loopn: nodes::LoopNode = nodes::LoopNode{
            block,
            expr: None,
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: Some(loopn),
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

    
        let n: Node = self.create_node(NodeType::LOOP, nodedat, pos);

        return n;        
    }
    
    fn parse_break(&mut self) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: String::from("")
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::BREAK, nodedat, pos);

        self.advance();
    
        return n;
    }
    
    fn parse_continue(&mut self) -> Node{
        let int: nodes::NumNode = nodes::NumNode{
            left: String::from("")
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
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };
    
        let n: Node = self.create_node(NodeType::CONTINUE, nodedat, pos);

        self.advance();
    
        return n;
    }

    fn parse_while(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();

        let expr: Node = self.expr(Precedence::Lowest);
        
        pos.endcol = expr.pos.endcol;
        
        self.skip_newline();
    
        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        self.skip_newline();
        
        let block: Vec<Node> = self.block();

        self.skip_newline();
        
        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        let loopn: nodes::LoopNode = nodes::LoopNode{
            block,
            expr: Some(expr),
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: Some(loopn),
            enumn: None,
            traitn: None,
            is: None,
            matchn: None,
        };

    
        let n: Node = self.create_node(NodeType::WHILE, nodedat, pos);

        return n;        
    }

    fn parse_enum(&mut self) -> Node {
        let pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: self.current.endcol,
        };

        self.advance();
    
        if !self.current_is_type(TokenType::IDENTIFIER) {
            self.raise_error("Expected name.", ErrorType::InvalidTok);
        }

        let name = self.current.data.clone();

        self.advance();

        self.skip_newline();
    
        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        self.skip_newline();

        let mut variants: Vec<String> = Vec::new();
        let mut tps: Vec<Option<Type>> = Vec::new();

        while self.current_is_type(TokenType::IDENTIFIER) {
            let name: String = self.current.data.clone();
            self.advance();

            variants.push(name);

            if self.current_is_type(TokenType::LT) {
                self.advance();

                tps.push(Some(self.parse_type(DataMutablility::Immutable).1));
        
                if !self.current_is_type(TokenType::GT) {
                    self.raise_error("Expected right parenthesis.", ErrorType::InvalidTok);
                }
                self.advance();
            }
            else {
                tps.push(None);
            }
        
            if !self.current_is_type(TokenType::COMMA) {
                self.raise_error("Expected comma.", ErrorType::InvalidTok);
            }
    
            self.advance();

            self.skip_newline();
        }

        self.skip_newline();
        
        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        let enumn: nodes::EnumNode = nodes::EnumNode{
            name,
            variants,
            tps,
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: Some(enumn),
            traitn: None,
            is: None,
            matchn: None,
        };

    
        let n: Node = self.create_node(NodeType::ENUM, nodedat, pos);

        return n;        
    }

    fn parse_trait(&mut self) -> Node {
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();
    
        if !self.current_is_type(TokenType::IDENTIFIER) {
            self.raise_error("Expected name.", ErrorType::InvalidTok);
        }

        let traitname = self.current.data.clone();

        pos.endcol = self.current.endcol;

        self.advance();

        self.skip_newline();
    
        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }
        
        self.advance();

        self.skip_newline();

        let mut functions: Vec<types::TemplateTraitSignature> = Vec::new();
        let mut vars: std::collections::HashMap<String, Type> = std::collections::HashMap::new();

        while !self.current_is_type(TokenType::RCURLY) && !self.current_is_type(TokenType::EOF) {
            if self.current_is_type(TokenType::IDENTIFIER) {
                let name: String = self.current.data.clone();

                self.advance();
    
                if !self.current_is_type(TokenType::COLON) {
                    self.raise_error("Expected colon.", ErrorType::InvalidTok);
                }

                self.advance();

                let tp: Type = self.parse_type(DataMutablility::Immutable).1;

                vars.insert(name, tp);

                self.skip_newline();
                
                if self.current_is_type(TokenType::RCURLY) {
                    break;
                }
                continue;
            }

            if !self.current_is_type(TokenType::KEYWORD) && self.current.data == "fn" {
                self.raise_error("Expected fn.", ErrorType::InvalidStatement);
            }

            self.advance();

            if !self.current_is_type(TokenType::IDENTIFIER) {
                self.raise_error("Expected identifier.", ErrorType::InvalidTok);
            }

            let name: String = self.current.data.clone();
            let methodname: Option<String> = None;
            let namespacename: Option<String> = None;
            let mut template_types: Vec<String> = Vec::new();

            self.advance();   

            if self.current_is_type(TokenType::LT) {
                self.advance();
                while self.current_is_type(TokenType::IDENTIFIER) {
                    template_types.push(self.current.data.clone());

                    self.advance();

                    if !self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::GT) {
                        self.raise_error("Expected comma.", ErrorType::InvalidTok);
                    }
                    self.advance();

                    if self.current_is_type(TokenType::GT) {
                        self.advance();
                        break;
                    }
                }
            }

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
            while !self.current_is_type(TokenType::RPAREN) && !self.current_is_type(TokenType::EOF) {
                args.args.push(self.parse_type(DataMutablility::Immutable).1);
                if !self.current_is_type(TokenType::COMMA) && !self.current_is_type(TokenType::RPAREN) {
                    self.raise_error("Expected comma.", ErrorType::InvalidTok);
                }

                if self.current_is_type(TokenType::RPAREN) {
                    break;
                }

                self.advance();

            }

            //



            if !self.current_is_type(TokenType::RPAREN) {
                self.raise_error("Expected right parenthesis.", ErrorType::InvalidTok);
            }

            self.advance();

            if self.current_is_type(TokenType::SMALLARROW) {
                self.advance();

                args.rettp.push(self.parse_type(DataMutablility::Immutable).1);

                self.backadvance();
                
                self.advance();
            }
            else {
                args.rettp.push(Type {
                    isfn: false,
                    isarr: false,
                    isdyn: false,
                    arrtp: None,
                    arrlen: None,
                    data: Some(String::from("void")),
                    args: None,
                    mutability: DataMutablility::Immutable,
                });
            }

            functions.push(types::TemplateTraitSignature {
                name,
                namespacename,
                methodname,
                template_types,
                args,
            });

            self.skip_newline();
            
        
            if self.current_is_type(TokenType::RCURLY) {
                break;
            }
        }

        self.skip_newline();
        
        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        let traitn: nodes::TraitNode = nodes::TraitNode{
            traitname,
            functions,
            vars,
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: Some(traitn),
            is: None,
            matchn: None,
        };

    
        let n: Node = self.create_node(NodeType::TRAIT, nodedat, pos);

        return n;     
    }

    fn parse_match(&mut self) -> Node{
        let mut pos = Position {
            line: self.current.line,
            startcol: self.current.startcol,
            endcol: 0,
        };

        self.advance();
        
        let expr: Node = self.expr(Precedence::Lowest);

        pos.endcol = self.current.endcol;
        
        self.skip_newline();

        if !self.current_is_type(TokenType::LCURLY) {
            self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        let mut patterns: Vec<(Node, Vec<Node>)> = Vec::new();

        while self.current_is_type(TokenType::IDENTIFIER) {
            let pattern: Node = self.generate_identifier(self.current.data.clone());
        
            self.skip_newline();

            if !self.current_is_type(TokenType::FATARROW) {
                self.raise_error("Expected fat arrow.", ErrorType::InvalidTok);
            }
    
            self.advance();

            self.skip_newline();

            if !self.current_is_type(TokenType::LPAREN) {
                self.raise_error("Expected left curly bracket.", ErrorType::InvalidTok);
            }
    
            self.advance();

            patterns.push((pattern, self.block()));

            self.skip_newline();

            if !self.current_is_type(TokenType::RPAREN) {
                self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
            }
    
            self.advance();

            self.skip_newline();
        }
        
        self.skip_newline();

        if !self.current_is_type(TokenType::RCURLY) {
            self.raise_error("Expected right curly bracket.", ErrorType::InvalidTok);
        }

        self.advance();

        let matchn: nodes::MatchNode = nodes::MatchNode{
            expr,
            patterns,
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
            unary: None,
            st: None,
            initst: None,
            attr: None,
            attrassign: None,
            str: None,
            arr: None,
            impln: None,
            ifn: None,
            loopn: None,
            enumn: None,
            traitn: None,
            is: None,
            matchn: Some(matchn),
        };

    
        let n: Node = self.create_node(NodeType::MATCH, nodedat, pos);

        return n;        
    }

}