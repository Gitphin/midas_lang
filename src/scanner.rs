use std::string::String;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(s: &str) -> Self {
        // construct scanner struct
        Self {
            source: s.to_string(), 
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
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

        if errs.len() > 0 {
            let mut join = String::new();
            errs.iter().for_each(|e| {
                join.push_str(&e);
                join.push_str("\n");
            });
        }
        // needs to be cloned (set Clone attr w/ derive)
        Ok(self.tokens.clone())
    }
    // checks if at end of line
    fn is_at_end (self: &Self) -> bool {
        self.current >= self.source.len()
    }

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
            _ => return Err(format!("Bad char at line {}: {}", self.line, c)),
        }
        Ok(())
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        c as char
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_p2(token_type, None);
    }

    fn add_token_p2(self: &mut Self, token_type: TokenType, literal: Option<LiteralVal>) {
        let mut text = String::new();
        let byte_rep = self.source.as_bytes();
        for i in self.start..self.current {
            text.push(byte_rep[i] as char)
        }

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_num: self.line,
        });

    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single Char
    LParen, RParen, LBrace, RBrace, 
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    // Single/Double Char
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,
    // Literals
    Identifier, String, Number,
    // Keywords
    And, Class, Else, False, Fun, For, If, Nil,
    Or, Print, Return, Super, This, True, Var, While,

    Eof
}

use TokenType::*;

#[derive(Debug, Clone)]
pub enum LiteralVal {
    IntVal(i64),
    FVal(f64),
    StringVal(String),
    IdentifierVal(String),
}
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

