use super::ExprNode;
use super::TypeName;
use std::collections::BTreeMap;
use sym::Sym;

#[derive(Debug)]
pub enum Literal {
    Nil,
    Integer(i64),
    Float(f64),
    String(String),
    Label(Sym),
    SimdLiteral(Vec<ExprNode>),
    ArrayLiteral(Vec<ExprNode>),
    ArraySplat(ExprNode, ExprNode),
    ObjectLiteral(Option<TypeName>, BTreeMap<Sym, Option<ExprNode>>),
}
