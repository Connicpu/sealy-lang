use super::Statement;

#[derive(Debug)]
pub struct Scope<'input> {
    pub statements: Vec<Statement<'input>>,
}
