use std::string::String;


fn is_digit(c: char) -> bool {
    return (c as u8) >= '0' as u8 && c as u8 <= '9' as u8
}
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    // Initialize scanner struct
    pub fn new(s: &str) -> Self {
        Self {
            source: s.to_string(), 
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
    // Scans the token information from struct
    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        // vec to check for errors
        let mut errs = vec![];
        while !self.is_at_end() {
            // beginning of next lexeme
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(e) => errs.push(e),
            }
        }
        // token creation
        self.tokens.push(Token {
            token_type: Eof, 
            lexeme: "".to_string(), 
            literal: None, 
            line_num: self.line});
        // makes err vec proper
        if errs.len() > 0 {
            let mut join = "".to_string();
            for e in errs {
                join.push_str(&e);
                join.push_str("\n");
            }
            return Err(join);
        }
        // needs to be cloned (set Clone attr w/ derive)
        Ok(self.tokens.clone())
    }
    // checks if at end of line
    fn is_at_end (self: &Self) -> bool {
        self.current >= self.source.len()
    }
    // Scans a token data char by char
    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(LParen),
            ')' => self.add_token(RParen),
            '{' => self.add_token(LBrace),
            '}' => self.add_token(RBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {let t = if self.match_char('=') {
                BangEqual 
            } else { 
                Bang 
            }; self.add_token(t);},
            '=' => {let t = if self.match_char('=') {
                EqualEqual 
            } else { 
                Equal 
            }; self.add_token(t);},
            '<' => {let t = if self.match_char('=') {
                LessEqual 
            } else { 
                Less
            }; self.add_token(t);},
            '>' => {let t = if self.match_char('=') {
                GreaterEqual 
            } else { 
                Greater 
            }; self.add_token(t);},
            '/' => { 
                if self.match_char('/') {
                    //NOTE: May have to make this in OR way
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }                      
                } else {
                    self.add_token(Slash);}},
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string()?,
            c => {
                if is_digit(c) {
                    self.number();
                } else {
                    return Err(format!("Bad char at line {}: {}", self.line, c));
                }
            }
            //NOTE: This may have to be changed later
        }
        Ok(())
    }
    fn number(self: &mut Self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let s = &self.source[self.start..self.current];
        let v = s.parse::<f64>();
        match v {
            Ok(v) => self.add_token_p2(Number, Some(FVal(v))),
            Err(_) => return Err(format!("Could not parse num: {}", s)),
        }
        Ok(())
    }
    // Peek at next value after current
    fn peek_next(self: &Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap()
    }
    // Handle string literals
    fn string(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            //TODO: Make this into an error somehow
            println!("Unterminated string at line {}", self.line);
            return Err("Unterminated string".to_string());
        }
        self.advance();

        // let mut text = "".to_string();
        // get byte rep of source so can index
        let v = &self.source[self.start + 1..self.current - 1];
        self.add_token_p2(StringLit, Some(StringVal(v.to_string())));
        Ok(())
    }
    // Does not modify, checks character at curr pointer
    fn peek(self: &Self) -> char {
        // if at end ret null terminator
        match self.is_at_end() {
            true => return '\0',
            _ => return self.source.chars().nth(self.current).unwrap()
        }
    }
    // Checks if next char is the expected val
    fn match_char(self: &mut Self, expect: char) -> bool {
       // if at end there should be no other char
       if self.is_at_end() {
           return false
       }
       // if not expected val ret false
       if self.source.chars().nth(self.current).unwrap() as char != expect {
           return false
       } else { 
       // incr curr pointer
           self.current +=1; return true 
       }
    }
    // Advances string index of the source 
    fn advance(self: &mut Self) -> char {
        //NOTE: Updating curr might have to be placed BEFORE let c
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }
    // First add_token call (not too sure but book says to do)
    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_p2(token_type, None);
    }
    // Add source to the lexeme
    fn add_token_p2(self: &mut Self, token_type: TokenType, literal: Option<LiteralVal>) {
        let mut text = "".to_string();
        // get byte rep of source so can index
        let _ = self.source[self.start..self.current].chars().map(|c| text.push(c));
        // add to token vec
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_num: self.line,
        });

    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single Char
    LParen, RParen, LBrace, RBrace, 
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    // Single/Double Char
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,
    // Literals
    Identifier, StringLit, Number,
    // Keywords
    And, Class, Else, False, Fun, For, If, Nil,
    Or, Print, Return, Super, This, True, Var, While,

    Eof
}

use crate::TokenType::*;

#[derive(Debug, Clone)]
pub enum LiteralVal {
    IntVal(i64),
    FVal(f64),
    StringVal(String),
    IdentifierVal(String),
}

use LiteralVal::*;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralVal>,
    line_num: usize,
}

// Need this to display the tokens w/ to_string
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    // Initialize token struct
    pub fn new_token(token_type: TokenType, lexeme: String, literal: Option<LiteralVal>, line_num: usize) -> Self {
        Self{token_type, lexeme, literal, line_num}
    }
    // Converts to string format
    pub fn to_string(self: &Self) -> String {
        // format is useful here to combine and return a string
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn one_char_tokens() {
//         let s = "!( )(";
//         let mut scanner = Scanner::new(s);
//         scanner.scan_tokens();

//         assert_eq!(scanner.tokens.len(), 5);
//         assert_eq!(scanner.tokens[0].token_type, Bang);
//     }
// }
