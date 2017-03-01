use super::ExprNode;
use sym::Sym;

#[derive(Debug)]
pub struct Lambda {
    pub parameters: Vec<Sym>,
    pub body: ExprNode,
    pub throws: bool,
}
