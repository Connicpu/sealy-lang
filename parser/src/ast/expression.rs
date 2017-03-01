use super::AssignOp;
use super::BinOp;
use super::ForLoop;
use super::IfChain;
use super::Lambda;
use super::Literal;
use super::ScopeNode;
use sym::Sym;

pub type ExprNode = super::Node<Expression>;

#[derive(Debug)]
pub enum Expression {
    Literal(Box<Literal>),
    UserSuffix(Box<UserSuffix>),
    NamedReference(Sym),
    EmptyTuple,
    Tuple(Box<Vec<ExprNode>>),
    MemberAccess(Box<MemberAccess>),
    TupleAccess(Box<TupleAccess>),
    ArrayIndex(Box<ArrayIndex>),
    FunctionCall(Box<FunctionCall>),

    Scope(Box<ScopeNode>),
    Lambda(Box<Lambda>),
    BinaryOp(Box<BinaryOp>),
    AssignmentOp(Box<AssignmentOp>),
    Throw(Box<ExprNode>),
    Break(Box<Break>),
    Continue(Box<Continue>),
    Return(Box<ExprNode>),
    Negate(Box<ExprNode>),
    Not(Box<ExprNode>),
    Try(Box<ExprNode>),

    For(Box<ForLoop>),
    If(Box<IfChain>),
}

#[derive(Debug)]
pub struct BinaryOp {
    pub lhs: ExprNode,
    pub rhs: ExprNode,
    pub operator: BinOp,
}

#[derive(Debug)]
pub struct AssignmentOp {
    pub lhs: ExprNode,
    pub rhs: ExprNode,
    pub operator: AssignOp,
}

#[derive(Debug)]
pub struct UserSuffix {
    pub value: Literal,
    pub suffix: Sym,
}

#[derive(Debug)]
pub struct MemberAccess {
    pub lhs: ExprNode,
    pub rhs: Sym,
}

#[derive(Debug)]
pub struct TupleAccess {
    pub lhs: ExprNode,
    pub rhs: i64,
}

#[derive(Debug)]
pub struct ArrayIndex {
    pub lhs: ExprNode,
    pub indices: Vec<ExprNode>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub lhs: ExprNode,
    pub parameters: Vec<ExprNode>,
}

#[derive(Debug)]
pub struct Continue {
    pub label: Option<Sym>,
    pub value: Option<ExprNode>,
}

#[derive(Debug)]
pub struct Break {
    pub label: Option<Sym>,
    pub value: Option<ExprNode>,
}
