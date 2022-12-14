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
pub struct IntNode {
    pub left: String,
}

impl std::fmt::Display for IntNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Integer '{}'", self.left)
    }    
}

#[derive(Clone)]
pub struct NodeData {
    pub binary: Option<BinaryNode>,
    pub int: Option<IntNode>,
}