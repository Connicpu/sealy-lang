use parser::ast::ExprNode;
use parser::ast::Node;
use parser::ast::ScopeNode;

#[derive(Debug)]
pub struct IfChain<'input> {
    pub items: Vec<Node<IfItem<'input>>>,
    pub else_block: Option<ScopeNode<'input>>,
}

impl<'input> Default for IfChain<'input> {
    fn default() -> Self {
        IfChain {
            items: vec![],
            else_block: None,
        }
    }
}

#[derive(Debug)]
pub struct IfItem<'input> {
    pub condition: ExprNode<'input>,
    //pub binding: Option<MatchPattern<'input>>,
    pub block: ScopeNode<'input>,
}
