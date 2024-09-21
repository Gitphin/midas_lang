use crate::expr::{Expr, Expr::*};
use crate::scanner::{Token, TokenType, TokenType::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // Construct the parser struct
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    // Expands to equality rule
    fn expression(&mut self) -> Expr {
        self.equality()
    }
    //Use our comparison func to assign expr, loop through w/ matching fn conditionals
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.matchings(&[BangEqual, EqualEqual]) {
            let operation = self.previous();
            let r: Expr = self.comparison();
            expr = Binary {
                op: operation,
                l: Box::from(expr),
                r: Box::from(r),
            };
        }
        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();
        while self.matchings(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operation: Token = self.previous();
            let r: Expr = self.term();
            expr = Binary {
                op: operation,
                l: Box::from(expr),
                r: Box::from(r),
            };
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.matchings(&[Minus, Plus]) {
            let operation: Token = self.previous();
            let r: Expr = self.term();
            expr = Binary {
                op: operation,
                l: Box::from(expr),
                r: Box::from(r),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.matchings(&[Slash, Star]) {
            let operation: Token = self.previous();
            let r: Expr = self.term();
            expr = Binary {
                op: operation,
                l: Box::from(expr),
                r: Box::from(r),
            };
        }
        expr
    }
    fn unary(&mut self) -> Expr {
        if self.matchings(&[Bang, Minus]) {
            let operation: Token = self.previous();
            let r: Expr = self.unary();
            return Unary {
                op: operation,
                r: Box::from(r),
            };
        }
        self.primary()
    }
    // Leave off on
    fn primary(&mut self) -> Expr {
        todo!()
    }

    // fn matching(&mut self, t: TokenType) -> bool {
    //     if self.is_at_end() {
    //         return false;
    //     } else {
    //         if self.peek().token_type == t {
    //             self.advance();
    //             return true;
    //         }
    //     }
    //     false
    // }
    // Checks for a != or == token, ret true if found, may need to replace with macro
    // due to how variadics work in Rust
    fn matchings(&mut self, types: &[TokenType]) -> bool {
        for &ttype in types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        false
    }
    // check our tokens and check for eof
    fn check(&mut self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == ttype
    }
    // Advance through the tokens, check if we are at the end if not incr cur pointer
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    // Checks if at end (hit an End of file token)
    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == Eof
    }
    // Gets current token
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
    // unfortunately have to use clone instead of passing as ref due to how mut works in Rust
    // Gets previous token
    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
