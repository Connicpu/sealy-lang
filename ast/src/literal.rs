use super::ExprNode;
use super::TypeName;
use std::collections::BTreeMap;
use std::rc::Rc;
use sym::Sym;

#[derive(Debug)]
pub enum Literal {
    Nil,
    Integer(i64),
    Float(f64),
    Symbol(Sym),
    String(Rc<String>),
    SimdLiteral(Box<Vec<ExprNode>>),
    ArrayLiteral(Box<Vec<ExprNode>>),
    ArraySplat(Box<ArraySplat>),
    ObjectLiteral(Box<ObjectLiteral>),
}

#[derive(Debug)]
pub struct ArraySplat {
    pub value: ExprNode,
    pub count: ExprNode,
}

#[derive(Debug)]
pub struct ObjectLiteral {
    pub type_constructor: Option<TypeName>,
    pub fields: BTreeMap<Sym, Option<ExprNode>>,
}
