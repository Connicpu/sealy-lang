use ast::BinaryOp;
use ast::ExprNode;
use ast::Expression;
use ast::Literal;
use ast::Node;

pub fn fold_expr(expr: Expression) -> Expression {
    use ast::Expression::*;

    match expr {
        Negate(box expr) => fold_negate(expr),
        BinaryOp(box op) => fold_binop(op),
        expr => expr,
    }
}

pub fn fold_negate(expr: ExprNode) -> Expression {
    let orig = loop {
        return match expr.node {
            Expression::Literal(box Literal::Integer(i)) => {
                Expression::Literal(box Literal::Integer(-i))
            }
            Expression::Literal(box Literal::Float(f)) => {
                Expression::Literal(box Literal::Float(-f))
            }
            expr => break expr,
        };
    };

    Expression::Negate(box Node::new(orig, expr.start, expr.end))
}

fn fold_binop(op: BinaryOp) -> Expression {
    Expression::BinaryOp(box op)
}
