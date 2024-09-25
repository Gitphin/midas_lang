#![allow(dead_code)]
#![allow(unused_variables)]
use crate::literals::LiteralVal::*;
use crate::scanner;
use crate::scanner::{Token, TokenType};

// Literal values class (enum because Rust)
#[derive(Clone, Debug, PartialEq)]
pub enum LiteralVal {
    NumVal(f32),
    StringVal(String),
    TrueVal,
    FalseVal,
    NullVal,
}

// Helper for token_fmt, just tries to unwrap value in Result type
fn unwrap_as_f32(literal: Option<scanner::LiteralVal>) -> f32 {
    match literal {
        Some(scanner::LiteralVal::IntVal(i)) => i as f32,
        Some(scanner::LiteralVal::FVal(i)) => i as f32,
        _ => panic!("Could not unwrap as f32"),
    }
}
// Helper for token_fmt, just tries to unwrap value in Result type
fn unwrap_as_str(literal: Option<scanner::LiteralVal>) -> String {
    match literal {
        Some(scanner::LiteralVal::StringVal(s)) => s.clone(),
        Some(scanner::LiteralVal::IdentifierVal(s)) => s.clone(),
        _ => panic!("Could not unwrap as String"),
    }
}

impl LiteralVal {
    // Formats the literal value as a string
    pub fn format_str(&self) -> String {
        match self {
            LiteralVal::NumVal(v) => v.to_string(),
            LiteralVal::StringVal(s) => (&s).to_string(),
            LiteralVal::TrueVal => "true".to_string(),
            LiteralVal::FalseVal => "false".to_string(),
            LiteralVal::NullVal => "null".to_string(),
        }
    }

    pub fn as_literal_type(&self) -> String {
        match self {
            LiteralVal::NumVal(_) => "Number".to_string(),
            LiteralVal::StringVal(_) => "String".to_string(),
            LiteralVal::TrueVal => "Boolean".to_string(),
            LiteralVal::FalseVal => "Boolean".to_string(),
            LiteralVal::NullVal => "Boolean".to_string(),
        }
    }
    // Formats the Token input into a Literal value
    pub fn token_fmt(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::NumVal(unwrap_as_f32(token.literal)),
            TokenType::StringLit => Self::StringVal(unwrap_as_str(token.literal)),
            TokenType::True => Self::TrueVal,
            TokenType::False => Self::FalseVal,
            TokenType::Null => Self::NullVal,
            _ => panic!("Failed to format token to literal"),
        }
    }
    // Truth evaluators for strings/nums (ex 0 or "" is not truthy)
    // TODO: Make lists not truthy? How am I gonna implement those haha
    pub fn is_falsy(&self) -> LiteralVal {
        match self {
            NumVal(x) => {
                if x.clone() == 0.0 as f32 {
                    TrueVal
                } else {
                    FalseVal
                }
            }
            StringVal(s) => {
                if s.len() == 0 {
                    TrueVal
                } else {
                    FalseVal
                }
            }
            TrueVal => FalseVal,
            FalseVal => TrueVal,
            NullVal => TrueVal,
        }
    }
    // Very simple bool truthy check to convert as a LiteralVal for booleans
    pub fn is_boolean_truthy(b: bool) -> Self {
        if b {
            TrueVal
        } else {
            FalseVal
        }
    }
}
