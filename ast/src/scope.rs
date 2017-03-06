use super::Node;
use super::Statement;

pub type ScopeNode = Node<Scope>;

#[derive(Debug)]
pub struct Scope {
    pub statements: Vec<Statement>,
}
