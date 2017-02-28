use parser::ast::DeclPattern;
use parser::ast::ExprNode;
use parser::ast::ScopeNode;

pub struct ForLoop<'input> {
    pub label: Option<&'input str>,
    pub binding: DeclPattern<'input>,
    pub expression: ExprNode<'input>,
    pub body: ScopeNode<'input>,
    pub else_body: Option<ScopeNode<'input>>,
}
