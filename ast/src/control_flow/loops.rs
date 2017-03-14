use ScopeNode;
use sym::Sym;

#[derive(Debug)]
pub struct Loop {
    pub label: Option<Sym>,
    pub block: ScopeNode,
}
