use ast::ExprNode;
use ast::Node;
use ast::ScopeNode;
use sym::Sym;

#[derive(Debug)]
pub struct IfChain {
    pub items: Vec<Node<IfItem>>,
    pub else_block: Option<ScopeNode>,
}

impl Default for IfChain {
    fn default() -> Self {
        IfChain {
            items: vec![],
            else_block: None,
        }
    }
}

#[derive(Debug)]
pub struct IfItem {
    pub condition: ExprNode,
    //pub binding: Option<MatchPattern>,
    pub block: ScopeNode,
}
