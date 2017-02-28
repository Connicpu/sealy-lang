use parser::ast::DeclPattern;
use parser::ast::Expression;
use parser::ast::Scope;

pub struct ForLoop<'input> {
    pub label: Option<&'input str>,
    pub binding: DeclPattern<'input>,
    pub expression: Expression<'input>,
    pub body: Scope<'input>,
    pub else_body: Option<Scope<'input>>,
}
