use crate::expr::{Expr, LiteralVal};

pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn interpret(&mut self, pe: Expr)->Result<LiteralVal, String> {
        pe.eval()
    }

}
