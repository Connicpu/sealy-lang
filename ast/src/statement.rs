use ExprNode;
use Node;
use TypeName;
use std::collections::BTreeMap;
use sym::Sym;

#[derive(Debug)]
pub enum Statement {
    Expression(ExprNode),
    VariableDecl(Box<Node<VariableDecl>>),
}

#[derive(Debug)]
pub struct VariableDecl {
    pub pattern: DeclPattern,
    pub type_bound: Option<TypeName>,
    pub expression: Option<ExprNode>,
}

#[derive(Debug)]
pub enum DeclPattern {
    Identifier(Sym),
    Tuple(Vec<DeclPattern>, bool),
    Array(Vec<DeclPattern>, bool),
    Simd(Vec<Sym>, bool),
    Object(BTreeMap<Sym, Option<DeclPattern>>),
}
