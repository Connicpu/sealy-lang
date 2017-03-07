use ScopeNode;
use sym::Sym;

#[derive(Debug)]
pub struct Loop {
    pub label: Sym,
    pub body: ScopeNode,
}
