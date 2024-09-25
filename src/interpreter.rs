#![allow(dead_code)]
#![allow(unused_variables)]
use crate::expr::Expr;
use crate::literals::LiteralVal;
use crate::statement::Statement;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn interpret(&mut self, pe: Expr) -> Result<LiteralVal, String> {
        pe.eval()
    }
    pub fn interpret_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Expression { expr } => {
                expr.eval()?;
            }
            Statement::Print { expr } => {
                let v = expr.eval()?;
                match v {
                    LiteralVal::NumVal(ref x) => println!("{}", x),
                    LiteralVal::StringVal(ref s) => println!("{}", s),
                    LiteralVal::TrueVal => println!("true"),
                    LiteralVal::FalseVal => println!("false"),
                    LiteralVal::NullVal => println!("null"),
                }
            }
        }
        Ok(())
    }
}
