use ExprNode;
use PatNode;
use ScopeNode;
use sym::Sym;

#[derive(Debug)]
pub struct While {
    pub label: Option<Sym>,
    pub expression: ExprNode,
    pub pattern: Option<PatNode>,
    pub block: ScopeNode,
    pub else_block: Option<ScopeNode>,
}
