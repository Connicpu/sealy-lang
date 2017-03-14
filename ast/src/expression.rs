use AssignOp;
use BinOp;
use ForLoop;
use IfChain;
use Lambda;
use Literal;
use Loop;
use Match;
use ScopeNode;
use TypeName;
use While;
use sym::Sym;

pub type ExprNode = super::Node<Expression>;

#[derive(Debug)]
pub enum Expression {
    NamedReference(Sym),
    Literal(Box<Literal>),
    UserSuffix(Box<UserSuffix>),
    Lambda(Box<Lambda>),
    Tuple(Box<Vec<ExprNode>>),
    EmptyTuple,

    MemberAccess(Box<MemberAccess>),
    TupleAccess(Box<TupleAccess>),
    ArrayIndex(Box<ArrayIndex>),
    FunctionCall(Box<FunctionCall>),
    ScopeResolution(Box<ScopeResolution>),
    ScopeGeneric(Box<ScopeGeneric>),

    BinaryOp(Box<BinaryOp>),
    AssignmentOp(Box<AssignmentOp>),

    Throw(Box<ExprNode>),
    Break(Box<Break>),
    Continue(Box<Continue>),
    Return(Box<ExprNode>),
    Negate(Box<ExprNode>),
    Not(Box<ExprNode>),
    Try(Box<ExprNode>),

    Scope(Box<ScopeNode>),
    For(Box<ForLoop>),
    If(Box<IfChain>),
    Loop(Box<Loop>),
    Match(Box<Match>),
    While(Box<While>),
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
}

#[derive(Debug)]
pub struct Break {
    pub label: Option<Sym>,
    pub value: Option<ExprNode>,
}

#[derive(Debug)]
pub struct ScopeResolution {
    pub lhs: ExprNode,
    pub rhs: Sym,
}

#[derive(Debug)]
pub struct ScopeGeneric {
    pub lhs: ExprNode,
    pub generics: Vec<TypeName>,
}
