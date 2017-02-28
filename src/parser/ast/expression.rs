use super::AssignOp;
use super::BinOp;
use super::Lambda;
use super::Literal;
use super::Scope;

#[derive(Debug)]
pub enum Expression<'input> {
    Literal(Box<Literal<'input>>),
    UserSuffix(Box<UserSuffix<'input>>),
    NamedReference(&'input str),
    EmptyTuple,
    Tuple(Box<Vec<Expression<'input>>>),
    MemberAccess(Box<MemberAccess<'input>>),
    TupleAccess(Box<TupleAccess<'input>>),
    ArrayIndex(Box<ArrayIndex<'input>>),
    FunctionCall(Box<FunctionCall<'input>>),

    Scope(Box<Scope<'input>>),
    Lambda(Box<Lambda<'input>>),
    BinaryOp(Box<BinaryOp<'input>>),
    AssignmentOp(Box<AssignmentOp<'input>>),
    Throw(Box<Expression<'input>>),
    Break(Box<Break<'input>>),
    Continue(Box<Continue<'input>>),
    Return(Box<Expression<'input>>),
    Negate(Box<Expression<'input>>),
    Not(Box<Expression<'input>>),
    Try(Box<Expression<'input>>),
}

#[derive(Debug)]
pub struct BinaryOp<'input> {
    pub lhs: Expression<'input>,
    pub rhs: Expression<'input>,
    pub operator: BinOp,
}

#[derive(Debug)]
pub struct AssignmentOp<'input> {
    pub lhs: Expression<'input>,
    pub rhs: Expression<'input>,
    pub operator: AssignOp,
}

#[derive(Debug)]
pub struct UserSuffix<'input> {
    pub value: Literal<'input>,
    pub suffix: &'input str,
}

#[derive(Debug)]
pub struct MemberAccess<'input> {
    pub lhs: Expression<'input>,
    pub rhs: &'input str,
}

#[derive(Debug)]
pub struct TupleAccess<'input> {
    pub lhs: Expression<'input>,
    pub rhs: i64,
}

#[derive(Debug)]
pub struct ArrayIndex<'input> {
    pub lhs: Expression<'input>,
    pub indices: Vec<Expression<'input>>,
}

#[derive(Debug)]
pub struct FunctionCall<'input> {
    pub lhs: Expression<'input>,
    pub parameters: Vec<Expression<'input>>,
}

#[derive(Debug)]
pub struct Continue<'input> {
    pub label: Option<&'input str>,
    pub value: Option<Expression<'input>>,
}

#[derive(Debug)]
pub struct Break<'input> {
    pub label: Option<&'input str>,
    pub value: Option<Expression<'input>>,
}
