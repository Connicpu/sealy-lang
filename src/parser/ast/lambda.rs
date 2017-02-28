use super::ExprNode;

#[derive(Debug)]
pub struct Lambda<'input> {
    pub parameters: Vec<&'input str>,
    pub body: ExprNode<'input>,
    pub throws: bool,
}
