use crate::scanner::Token;

pub enum LiteralVal {
    NumVal(f32),
    StringVal(String),
    TrueVal,
    FalseVal,
    Nil,
}

pub enum Expr {
    Binary {left: Box<Expr>, op: Token, right: Box<Expr>},
    Grouping {expr: Box<Expr>},
    Literal {val: LiteralVal},
    Unary {op: Token, right: Box<Expr>}
}

impl Expr {
    pub fn format_it(&self) -> String {
        match self {
            Expr::Binary {left, op, right} => todo!(),
            Expr::Grouping {expr} => todo!(),
            Expr::Literal {val} => todo!(),
            Expr::Unary {op, right} => todo!()
        }
    }
}

// Do more later am sleepy
