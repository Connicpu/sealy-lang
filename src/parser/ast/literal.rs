use super::Expression;
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
    SimdLiteral(Vec<Expression<'input>>),
    ArrayLiteral(Vec<Expression<'input>>),
    ArraySplat(Expression<'input>, Expression<'input>),
    ObjectLiteral(Option<TypeName<'input>>, BTreeMap<&'input str, Option<Expression<'input>>>),
}
