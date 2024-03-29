#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOpType {
    ADD,
    SUB,
    MUL,
    DIV,
    GT,
    GTE,
    LT,
    LTE,
    EQ,
    NE,
}

impl std::fmt::Display for BinaryOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BinaryOpType::ADD => write!(f, "+"),
            BinaryOpType::SUB => write!(f, "-"),
            BinaryOpType::MUL => write!(f, "*"),
            BinaryOpType::DIV => write!(f, "/"),
            BinaryOpType::GT => write!(f, ">"),
            BinaryOpType::GTE => write!(f, ">="),
            BinaryOpType::LT => write!(f, "<"),
            BinaryOpType::LTE => write!(f, "<="),
            BinaryOpType::EQ => write!(f, "=="),
            BinaryOpType::NE => write!(f, "!="),
        }
    }    
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnaryOpType {
    POS,
    NEG,
    REF,
    STMT,
}

impl std::fmt::Display for UnaryOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UnaryOpType::NEG => write!(f, "-"),
            UnaryOpType::REF => write!(f, "&"),
            UnaryOpType::POS => write!(f, "+"),
            UnaryOpType::STMT => write!(f, ";"),
        }
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryNode{
    pub left: crate::parser::Node,
    pub op: BinaryOpType,
    pub right: crate::parser::Node,
    pub isassign: bool,
}

impl std::fmt::Display for BinaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binary '{}' {} '{}'", self.left, self.op, self.right)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumNode {
    pub left: String,
}

impl std::fmt::Display for NumNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Num '{}'", self.left)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct LetNode {
    pub name: String,
    pub expr: Option<crate::parser::Node>,
    pub mutability: crate::codegen::types::DataMutablility,
    pub tp: Option<crate::parser::Type>,
}

impl std::fmt::Display for LetNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Let '{}' = {:?}", self.name, self.expr)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierNode {
    pub name: String,
}

impl std::fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier '{}'", self.name)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncNode {
    pub name: String,
    pub blocks: Vec<crate::parser::Node>,
    pub args: crate::parser::Args,
    pub methodname: Option<String>,
    pub namespacename: Option<String>,
    pub template_types: Vec<String>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct AssignNode {
    pub name: String,
    pub expr: crate::parser::Node,
}

impl std::fmt::Display for AssignNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' = {}", self.name, self.expr)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallNode {
    pub name: crate::parser::Node,
    pub args: Vec<crate::parser::Node>,
}

impl std::fmt::Display for CallNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call '{}'", self.name)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnNode {
    pub expr: Option<crate::parser::Node>,
}

impl std::fmt::Display for ReturnNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return '{:?}'", self.expr)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToNode {
    pub left: crate::parser::Node,
    pub tp: crate::parser::Type,
}

impl std::fmt::Display for ToNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' to '{}'", self.left, if self.tp.isfn {"fn"} else {self.tp.data.as_ref().unwrap().as_str()})
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct AsNode {
    pub left: crate::parser::Node,
    pub tp: crate::parser::Type,
}

impl std::fmt::Display for AsNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' as '{}'", self.left, if self.tp.isfn {"fn"} else {self.tp.data.as_ref().unwrap().as_str()})
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryNode{
    pub op: UnaryOpType,
    pub right: crate::parser::Node,
}

impl std::fmt::Display for UnaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unary {} '{}'", self.op, self.right)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructNode{
    pub name: String,
    pub names: Vec<String>,
    pub members: std::collections::HashMap<String, crate::parser::Type>,
}

impl std::fmt::Display for StructNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Struct '{}' of {} members", self.name, self.members.len())
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructInitNode{
    pub name: String,
    pub members: std::collections::HashMap<String, crate::parser::Node>,
    pub members_vec: Vec<String>,
}

impl std::fmt::Display for StructInitNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initialize struct '{}' of {} members", self.name, self.members.len())
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttrNode{
    pub name: crate::parser::Node,
    pub attr: String,
    pub expr: Option<crate::parser::Node>,
    pub template_types: Option<Vec<crate::parser::Type>>,
}

impl std::fmt::Display for AttrNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.name, self.attr)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttrAssignNode{
    pub name: crate::parser::Node,
    pub attr: String,
    pub expr: crate::parser::Node,
}

impl std::fmt::Display for AttrAssignNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{} = {}", self.name, self.attr, self.expr)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringNode{
    pub data: String,
}

impl std::fmt::Display for StringNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "String \"{}\"", self.data)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayNode{
    pub elements: Vec<crate::parser::Node>,
}

impl std::fmt::Display for ArrayNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Array of {} element(s)", self.elements.len())
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplNode{
    pub functions: Vec<crate::parser::Node>,
    pub traitnm: String,
    pub structnm: String,
}

impl std::fmt::Display for ImplNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Implement \"{}\" for \"{}\"", self.traitnm, self.structnm)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfNode{
    pub ifs: Vec<(crate::parser::Node, Vec<crate::parser::Node>)>,
    pub else_opt: Option<Vec<crate::parser::Node>>,
    pub inexpr: bool,
}

impl std::fmt::Display for IfNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "If")
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoopNode{
    pub block: Vec<crate::parser::Node>,
    pub expr: Option<crate::parser::Node>,
}

impl std::fmt::Display for LoopNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Loop (expr={:?}) {{", self.expr)?;
        for node in self.block.clone() {
            writeln!(f, "    {}", node)?;
        }
        write!(f, "    }}")
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumNode{
    pub variants: Vec<String>,
    pub name: String,
    pub tps: Vec<Option<crate::parser::Type>>,
    pub template_types: Vec<String>,
}

impl std::fmt::Display for EnumNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Enum '{}'", self.name)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct TraitNode{
    pub traitname: String,
    pub functions: Vec<crate::codegen::types::TemplateTraitSignature>,
    pub vars: std::collections::HashMap<String, crate::parser::Type>,
}

impl std::fmt::Display for TraitNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Trait '{}'", self.traitname)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct IsNode{
    pub left: crate::parser::Node,
    pub variant: crate::parser::Node,
}

impl std::fmt::Display for IsNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Is '{}' '{}'", self.left, self.variant)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchNode{
    pub expr: crate::parser::Node,
    pub patterns: Vec<(Option<crate::parser::Node>, Option<String>, Vec<crate::parser::Node>)>,
    pub inexpr: bool,
    pub have_default: bool,
}

impl std::fmt::Display for MatchNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Match for '{}'", self.expr)
    }    
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamespaceAttrNode{
    pub name: crate::parser::Node,
    pub attrs: Vec<String>,
    pub expr: Option<crate::parser::Node>,
    pub template_types: Option<Vec<crate::parser::Type>>,
}

impl std::fmt::Display for NamespaceAttrNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:?}", self.name, self.attrs)
    }    
}

#[derive(Clone, Debug, PartialEq)]
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
    pub initst: Option<StructInitNode>,
    pub attr: Option<AttrNode>,
    pub attrassign: Option<AttrAssignNode>,
    pub str: Option<StringNode>,
    pub arr: Option<ArrayNode>,
    pub impln: Option<ImplNode>,
    pub ifn: Option<IfNode>,
    pub loopn: Option<LoopNode>,
    pub enumn: Option<EnumNode>,
    pub traitn: Option<TraitNode>,
    pub is: Option<IsNode>,
    pub matchn: Option<MatchNode>,
    pub nameattr: Option<NamespaceAttrNode>,
}