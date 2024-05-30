use crate::scanner::{Token, TokenType, TokenType::*};
use crate::expr::{Expr, Expr::*};


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // Construct the parser struct
    pub fn new(tokens: Vec<Token>) -> Self  {
        Self { 
        tokens,
        current: 0
        }
    }   
    // Expands to equality rule
    fn expression(&mut self) -> Expr { 
        self.equality()
    }
    //Use our comparison func to assign expr, loop through w/ matching fn conditionals 
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.matching(&vec![BangEqual, BangEqual]) {
            let op2: Token = self.previous();
            let r2: Expr = self.comparison();
            expr = Binary {op: op2, l: Box::from(expr), r: Box::from(r2)};
        }
    expr
    }
    fn comparison(&mut self) -> Expr {
        todo!()
    }
    fn previous(&mut self) -> Token {
        todo!()
    }
    // Checks for a != or == token, ret true if found
    fn matching (&mut self, types: &[TokenType]) -> bool {
        for &ttype in types {
            if self.check(ttype) {
                self.advance();
                return true;
            } 
        }
        false
    }
    fn check (&mut self, ttype: TokenType) -> bool {
        if self.is_at_end() {return false;}
        self.peek().token_type == ttype
    }
    fn advance(&mut self) {todo!()}
    fn is_at_end (&mut self) -> bool {todo!()}
    fn peek (&mut self) -> Token {todo!()}
}
