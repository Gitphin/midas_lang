use crate::expr::Expr;
use crate::Token;

pub enum Statement {
    Expression {
        expr: Expr
    },
    Print {
        expr: Expr
    },
    Var {
        t: Token,
        init: Expr
    }
}
