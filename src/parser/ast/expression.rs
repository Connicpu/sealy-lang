use super::AssignOp;
use super::BinOp;
use super::Lambda;
use super::Literal;
use super::ScopeNode;

pub type ExprNode<'input> = super::Node<Expression<'input>>;

#[derive(Debug)]
pub enum Expression<'input> {
    Literal(Box<Literal<'input>>),
    UserSuffix(Box<UserSuffix<'input>>),
    NamedReference(&'input str),
    EmptyTuple,
    Tuple(Box<Vec<ExprNode<'input>>>),
    MemberAccess(Box<MemberAccess<'input>>),
    TupleAccess(Box<TupleAccess<'input>>),
    ArrayIndex(Box<ArrayIndex<'input>>),
    FunctionCall(Box<FunctionCall<'input>>),

    Scope(Box<ScopeNode<'input>>),
    Lambda(Box<Lambda<'input>>),
    BinaryOp(Box<BinaryOp<'input>>),
    AssignmentOp(Box<AssignmentOp<'input>>),
    Throw(Box<ExprNode<'input>>),
    Break(Box<Break<'input>>),
    Continue(Box<Continue<'input>>),
    Return(Box<ExprNode<'input>>),
    Negate(Box<ExprNode<'input>>),
    Not(Box<ExprNode<'input>>),
    Try(Box<ExprNode<'input>>),
}

#[derive(Debug)]
pub struct BinaryOp<'input> {
    pub lhs: ExprNode<'input>,
    pub rhs: ExprNode<'input>,
    pub operator: BinOp,
}

#[derive(Debug)]
pub struct AssignmentOp<'input> {
    pub lhs: ExprNode<'input>,
    pub rhs: ExprNode<'input>,
    pub operator: AssignOp,
}

#[derive(Debug)]
pub struct UserSuffix<'input> {
    pub value: Literal<'input>,
    pub suffix: &'input str,
}

#[derive(Debug)]
pub struct MemberAccess<'input> {
    pub lhs: ExprNode<'input>,
    pub rhs: &'input str,
}

#[derive(Debug)]
pub struct TupleAccess<'input> {
    pub lhs: ExprNode<'input>,
    pub rhs: i64,
}

#[derive(Debug)]
pub struct ArrayIndex<'input> {
    pub lhs: ExprNode<'input>,
    pub indices: Vec<ExprNode<'input>>,
}

#[derive(Debug)]
pub struct FunctionCall<'input> {
    pub lhs: ExprNode<'input>,
    pub parameters: Vec<ExprNode<'input>>,
}

#[derive(Debug)]
pub struct Continue<'input> {
    pub label: Option<&'input str>,
    pub value: Option<ExprNode<'input>>,
}

#[derive(Debug)]
pub struct Break<'input> {
    pub label: Option<&'input str>,
    pub value: Option<ExprNode<'input>>,
}
