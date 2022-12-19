#[derive(Clone, Copy)]
pub enum BinaryOpType {
    DEFAULT,
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
            _ => write!(f, ""),
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
pub struct I32Node {
    pub left: String,
}

impl std::fmt::Display for I32Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i32 '{}'", self.left)
    }    
}

#[derive(Clone)]
pub struct LetNode {
    pub name: String,
    pub expr: crate::parser::Node,
    pub mutability: crate::codegen::types::DataMutablility,
    pub tp: Option<crate::parser::Arg>,
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
pub struct NodeData {
    pub binary: Option<BinaryNode>,
    pub int: Option<I32Node>,
    pub letn: Option<LetNode>,
    pub identifier: Option<IdentifierNode>,
    pub func: Option<FuncNode>,
    pub assign: Option<AssignNode>,
    pub call: Option<CallNode>,
}