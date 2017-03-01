use super::ExprNode;
use super::TypeName;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Literal<'input> {
    Nil,
    Integer(i64),
    Float(f64),
    String(Cow<'input, str>),
    Label(&'input str),
    SimdLiteral(Vec<ExprNode<'input>>),
    ArrayLiteral(Vec<ExprNode<'input>>),
    ArraySplat(ExprNode<'input>, ExprNode<'input>),
    ObjectLiteral(Option<TypeName<'input>>, BTreeMap<&'input str, Option<ExprNode<'input>>>),
}
