use super::Expression;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Statement<'input> {
    Expression(Expression<'input>),
    VariableDecl(Box<VariableDecl<'input>>),
}

#[derive(Debug)]
pub struct VariableDecl<'input> {
    pub pattern: DeclPattern<'input>,
    pub expression: Option<Expression<'input>>,
}

#[derive(Debug)]
pub enum DeclPattern<'input> {
    Identifier(&'input str),
    Tuple(Vec<DeclPattern<'input>>, bool),
    Array(Vec<DeclPattern<'input>>, bool),
    Simd(Vec<&'input str>, bool),
    Object(BTreeMap<&'input str, Option<DeclPattern<'input>>>),
}
