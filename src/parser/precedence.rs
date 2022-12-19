#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Precedence {
    Lowest,
    Assign,
    LogicalOr,   
    LogicalAnd,
    LogicalNot, 
    Equals,         
    Comparison,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    BitwiseShift,
    Sum,          
    Product, 
    BitwiseNot,
    Exp, 
    Call,        
    Index,
    Unary,
    Ternary,   
}