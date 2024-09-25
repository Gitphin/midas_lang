#![allow(dead_code)]
#![allow(unused_variables)]
use crate::enviro::Enviro;
use crate::expr::Expr;
use crate::literals::*;
use crate::statement::Statement;

pub struct Interpreter {
    enviro: Enviro,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            enviro: Enviro::new(),
        }
    }
    pub fn interpret(&mut self, pe: Expr) -> Result<LiteralVal, String> {
        pe.eval(&self.enviro)
    }
    pub fn interpret_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Expression { expr } => {
                expr.eval(&self.enviro)?;
            }
            Statement::Var { t, init } => {
                let val = init.eval(&self.enviro)?;
                self.enviro.define(t.lexeme, val);
            }
            Statement::Print { expr } => {
                let v = expr.eval(&self.enviro)?;
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
