use ExprNode;
use Literal;
use Node;
use TypeName;
use sym::Sym;

#[derive(Debug)]
pub struct Match {
    pub expression: ExprNode,
    pub patterns: Vec<MatchPattern>,
}

#[derive(Debug)]
pub enum MatchPattern {
    Any,
    AnySplat,
    Variable(Sym),
    Literal(Literal),
    Tuple {
        enum_type: Option<TypeName>,
        items: Vec<Node<MatchPattern>>,
    },
    Object {
        object_type: Option<TypeName>,
        items: Vec<Node<(Sym, Option<Node<MatchPattern>>)>>,
    },
    Array(Vec<Node<MatchPattern>>),
    Simd(Vec<Node<MatchPattern>>),
}
