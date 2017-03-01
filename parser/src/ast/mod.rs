pub use self::control_flow::*;
pub use self::expression::*;
pub use self::items::*;
pub use self::lambda::*;
pub use self::literal::*;
pub use self::ops::*;
pub use self::scope::*;
pub use self::statement::*;
pub use self::type_name::*;

use lexer::Location;

pub mod control_flow;
pub mod expression;
pub mod items;
pub mod lambda;
pub mod literal;
pub mod ops;
pub mod scope;
pub mod statement;
pub mod type_name;

#[derive(Debug)]
pub struct Node<T> {
    pub node: T,
    pub start: Location,
    pub end: Location,
}

impl<T> Node<T> {
    pub fn new(node: T, start: Location, end: Location) -> Node<T> {
        Node {
            node: node,
            start: start,
            end: end,
        }
    }

    pub fn unwrap(self) -> (Location, T, Location) {
        (self.start, self.node, self.end)
    }
}

impl<T> From<(Location, T, Location)> for Node<T> {
    fn from((s, n, e): (Location, T, Location)) -> Self {
        Node::new(n, s, e)
    }
}
