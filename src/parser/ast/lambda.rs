use super::Expression;

#[derive(Debug)]
pub struct Lambda<'input> {
    pub parameters: Vec<&'input str>,
    pub body: Expression<'input>,
}
