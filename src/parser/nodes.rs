#[derive(Clone, Copy)]
pub enum BinaryOpType {
    ADD,
    SUB,
    MUL,
    DIV,
}

impl std::fmt::Display for BinaryOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BinaryOpType::ADD => write!(f, "+"),
            BinaryOpType::SUB => write!(f, "-"),
            BinaryOpType::MUL => write!(f, "*"),
            BinaryOpType::DIV => write!(f, "/"),
        }
    }    
}

#[derive(Clone, Copy)]
pub enum UnaryOpType {
    POS,
    NEG,
    REF,
}

impl std::fmt::Display for UnaryOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UnaryOpType::NEG => write!(f, "-"),
            UnaryOpType::REF => write!(f, "&"),
            UnaryOpType::POS => write!(f, "+"),
        }
    }    
}

#[derive(Clone)]
pub struct BinaryNode{
    pub left: crate::parser::Node,
    pub op: BinaryOpType,
    pub right: crate::parser::Node,
}

impl std::fmt::Display for BinaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binary '{}' {} '{}'", self.left, self.op, self.right)
    }    
}

#[derive(Clone)]
pub struct NumNode {
    pub left: String,
}

impl std::fmt::Display for NumNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Num '{}'", self.left)
    }    
}

#[derive(Clone)]
pub struct LetNode {
    pub name: String,
    pub expr: crate::parser::Node,
    pub mutability: crate::codegen::types::DataMutablility,
    pub tp: Option<crate::parser::Type>,
}

impl std::fmt::Display for LetNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Let '{}' = {}", self.name, self.expr)
    }    
}

#[derive(Clone)]
pub struct IdentifierNode {
    pub name: String,
}

impl std::fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier '{}'", self.name)
    }    
}

#[derive(Clone)]
pub struct FuncNode {
    pub name: String,
    pub blocks: Vec<crate::parser::Node>,
    pub args: crate::parser::Args,
}

impl std::fmt::Display for FuncNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Function '{}' {{", self.name)?;
        for node in self.blocks.clone() {
            writeln!(f, "    {}", node)?;
        }
        write!(f, "    }}")
    }    
}

#[derive(Clone)]
pub struct AssignNode {
    pub name: String,
    pub expr: crate::parser::Node,
}

impl std::fmt::Display for AssignNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' = {}", self.name, self.expr)
    }    
}

#[derive(Clone)]
pub struct CallNode {
    pub name: crate::parser::Node,
    pub args: Vec<crate::parser::Node>,
}

impl std::fmt::Display for CallNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call '{}'", self.name)
    }    
}

#[derive(Clone)]
pub struct ReturnNode {
    pub expr: crate::parser::Node,
}

impl std::fmt::Display for ReturnNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return '{}'", self.expr)
    }    
}

#[derive(Clone)]
pub struct ToNode {
    pub left: crate::parser::Node,
    pub tp: crate::parser::Type,
}

impl std::fmt::Display for ToNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' to '{}'", self.left, if self.tp.isfn {"fn"} else {self.tp.data.as_ref().unwrap().as_str()})
    }    
}

#[derive(Clone)]
pub struct AsNode {
    pub left: crate::parser::Node,
    pub tp: crate::parser::Type,
}

impl std::fmt::Display for AsNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' as '{}'", self.left, if self.tp.isfn {"fn"} else {self.tp.data.as_ref().unwrap().as_str()})
    }    
}

#[derive(Clone)]
pub struct UnaryNode{
    pub op: UnaryOpType,
    pub right: crate::parser::Node,
}

impl std::fmt::Display for UnaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unary {} '{}'", self.op, self.right)
    }    
}

#[derive(Clone)]
pub struct StructNode{
    pub name: String,
    pub members: std::collections::HashMap<String, crate::parser::Type>,
}

impl std::fmt::Display for StructNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Struct '{}' of {} members", self.name, self.members.len())
    }    
}

#[derive(Clone)]
pub struct NodeData {
    pub binary: Option<BinaryNode>,
    pub num: Option<NumNode>,
    pub letn: Option<LetNode>,
    pub identifier: Option<IdentifierNode>,
    pub func: Option<FuncNode>,
    pub assign: Option<AssignNode>,
    pub call: Option<CallNode>,
    pub ret: Option<ReturnNode>,
    pub to: Option<ToNode>,
    pub unary: Option<UnaryNode>,
    pub st: Option<StructNode>,
}