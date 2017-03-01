use ast::DeclPattern;
use ast::ExprNode;
use ast::ScopeNode;
use sym::Sym;

#[derive(Debug)]
pub struct ForLoop {
    pub label: Option<Sym>,
    pub binding: DeclPattern,
    pub expression: ExprNode,
    pub body: ScopeNode,
    pub else_body: Option<ScopeNode>,
}
