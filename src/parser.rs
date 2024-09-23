use crate::expr::{Expr, Expr::*, LiteralVal};
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
    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    // Expands to equality rule
    pub fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }
    //Use our comparison func to assign expr, loop through w/ matching fn conditionals
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.matchings(&[BangEqual, EqualEqual]) {
            let operation = self.previous();
            let r: Expr = self.comparison()?;
            expr = Binary {
                l: Box::from(expr),
                op: operation,
                r: Box::from(r),
            };
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.term()?;
        while self.matchings(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operation: Token = self.previous();
            let r: Expr = self.term()?;
            expr = Binary {
                l: Box::from(expr),
                op: operation,
                r: Box::from(r),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.factor()?;

        while self.matchings(&[Minus, Plus]) {
            let operation: Token = self.previous();
            let r: Expr = self.factor()?;
            expr = Binary {
                l: Box::from(expr),
                op: operation,
                r: Box::from(r),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.unary()?;

        while self.matchings(&[Slash, Star]) {
            let operation: Token = self.previous();
            let r: Expr = self.unary()?;
            expr = Binary {
                l: Box::from(expr),
                op: operation,
                r: Box::from(r),
            };
        }
        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expr, String> {
        if self.matchings(&[Bang, Minus]) {
            let operation: Token = self.previous();
            let r: Expr = self.unary()?;
            return Ok(Unary {
                op: operation,
                r: Box::from(r),
            });
        }
        Ok(self.primary()?)
    }
    // Leave off on
    fn primary(&mut self) -> Result<Expr, String> {
        let t = self.peek();
        let res;
        match t.token_type {
            LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RParen, "Expecting ')' after expression")?;
                res = Grouping {
                    expr: Box::from(expr),
                }
            }
            True | False | Null | Number | StringLit => {
                self.advance();
                res = Literal {
                    val: LiteralVal::token_fmt(t),
                }
            }

            _ => return Err("Expected expression".to_string()),
        };
        Ok(res)
    }
    fn consume(&mut self, ttype: TokenType, err: &str) -> Result<(), String> {
        let t = self.peek();
        if t.token_type == ttype {
            self.advance();
            Ok(())
        } else {
            Err(err.to_string())
        }
    }
    fn matching(&mut self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            if self.peek().token_type == t {
                self.advance();
                return true;
            }
        }
        false
    }
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                ()
            }
            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => self.advance(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{LiteralVal::*, Scanner};
    #[test]
    fn testing_parser() {
        let o = Token {
            token_type: Number,
            lexeme: "1".to_string(),
            literal: Some(IntVal(1)),
            line_num: 1,
        };
        let p = Token {
            token_type: Star,
            lexeme: "*".to_string(),
            literal: None,
            line_num: 1,
        };
        let t = Token {
            token_type: Number,
            lexeme: "2".to_string(),
            literal: Some(IntVal(2)),
            line_num: 1,
        };
        let s = Token {
            token_type: Semicolon,
            lexeme: ";".to_string(),
            literal: None,
            line_num: 1,
        };

        let tokens = vec![o, p, t, s];
        let mut p: Parser = Parser::new(tokens);

        let pe = p.parse().unwrap();
        let se = pe.format_str();

        assert_eq!(se, "(* 1 2)");
    }

    #[test]
    fn parser_test2() {
        let src = "8 - 2 == 5 + 1";
        let mut s = Scanner::new(src);
        let t = s.scan_tokens().unwrap();
        let mut p = Parser::new(t);
        let pe = p.parse().unwrap();
        let se = pe.format_str();

        assert_eq!(se, "(== (- 8 2) (+ 5 1))");
    }
}

