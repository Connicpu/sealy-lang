use super::Node;
use super::Statement;

pub type ScopeNode<'input> = Node<Scope<'input>>;

#[derive(Debug)]
pub struct Scope<'input> {
    pub statements: Vec<Statement<'input>>,
}
