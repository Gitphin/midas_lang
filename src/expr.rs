use crate::scanner;
use crate::scanner::{Token, TokenType};
// use std::process::exit;
// use std::env;
// use std::fs::File;
// use std::io::{BufWriter, Write};

// Literal value class (converted to enum for Rust implementation)
#[derive(Clone, Debug, PartialEq)]
pub enum LiteralVal {
    NumVal(f32),
    StringVal(String),
    TrueVal,
    FalseVal,
    NullVal,
}

use LiteralVal::*;

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
// AST expression implementation
pub enum Expr {
    Binary {
        l: Box<Expr>,
        op: Token,
        r: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        val: LiteralVal,
    },
    Unary {
        op: Token,
        r: Box<Expr>,
    },
}

impl Expr {
    // Matches expr and formats to print correctly as a string
    pub fn format_str(&self) -> String {
        match self {
            Expr::Binary { l, op, r } => {
                format!("({} {} {})", op.lexeme, l.format_str(), r.format_str())
            }
            Expr::Grouping { expr } => format!("(group {})", expr.format_str()),
            Expr::Literal { val } => format!("{}", val.format_str()),
            Expr::Unary { op, r } => format!("({} {})", op.lexeme, r.format_str()),
        }
    }
    // Prints structure of syntax tree (useful for debugging)
    pub fn pretty_print(&self) {
        println!("{}", self.format_str());
    }
    // This acts as my interpeter, evaluates expressions
    pub fn eval(&self) -> Result<LiteralVal, String> {
        match self {
            Expr::Literal { val } => Ok(val.clone()),
            Expr::Grouping { expr } => expr.eval(),
            Expr::Unary { op, r } => {
                let r = r.eval()?;
                match (op.token_type, r.clone()) {
                    (TokenType::Minus, NumVal(x)) => Ok(NumVal(-x)),
                    // TODO: add for floats
                    (TokenType::Minus, _) => Err(format!(
                        "Cannot use Minus operator on type {}",
                        r.as_literal_type()
                    )),
                    (TokenType::Bang, any) => Ok(any.is_falsy()),
                    (tt, _) => Err(format!("{} is not a valid operator for unaries", tt)),
                }
            }
            // TODO: Keep adding stuff for this
            Expr::Binary { l, op, r } => {
                let l = l.eval()?;
                let r = r.eval()?;
                match (l.clone(), op.token_type, r.clone()) {
                    (NumVal(x), TokenType::Plus, NumVal(y)) => Ok(NumVal(x + y)),
                    (NumVal(x), TokenType::Minus, NumVal(y)) => Ok(NumVal(x - y)),
                    (NumVal(x), TokenType::Slash, NumVal(y)) => Ok(NumVal(x / y)),
                    (NumVal(x), TokenType::Star, NumVal(y)) => Ok(NumVal(x * y)),

                    (NumVal(x), TokenType::Greater, NumVal(y)) => {
                        Ok(LiteralVal::is_boolean_truthy(x > y))
                    }
                    (NumVal(x), TokenType::GreaterEqual, NumVal(y)) => {
                        Ok(LiteralVal::is_boolean_truthy(x >= y))
                    }
                    (NumVal(x), TokenType::Less, NumVal(y)) => {
                        Ok(LiteralVal::is_boolean_truthy(x < y))
                    }
                    (NumVal(x), TokenType::LessEqual, NumVal(y)) => {
                        Ok(LiteralVal::is_boolean_truthy(x <= y))
                    }

                    (StringVal(s), TokenType::Plus, StringVal(s2)) => {
                        Ok(StringVal(format!("{}{}", s, s2)))
                    }
                    (StringVal(s), TokenType::Greater, StringVal(s2)) => {
                        Ok(LiteralVal::is_boolean_truthy(s > s2))
                    }
                    (StringVal(s), TokenType::GreaterEqual, StringVal(s2)) => {
                        Ok(LiteralVal::is_boolean_truthy(s >= s2))
                    }
                    (StringVal(s), TokenType::Less, StringVal(s2)) => {
                        Ok(LiteralVal::is_boolean_truthy(s < s2))
                    }
                    (StringVal(s), TokenType::LessEqual, StringVal(s2)) => {
                        Ok(LiteralVal::is_boolean_truthy(s <= s2))
                    }

                    (NumVal(x), op, StringVal(s)) => Err(format!(
                        "Cannot use {} operater between Number and String types -=({} and {})=-",
                        op, x, s
                    )),
                    (StringVal(s), op, NumVal(x)) => Err(format!(
                        "Cannot use {} operater between String and Number types -=({} and {})=-",
                        op, s, x
                    )),

                    (x, TokenType::BangEqual, y) => Ok(LiteralVal::is_boolean_truthy(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(LiteralVal::is_boolean_truthy(x == y)),
                    (x, op, y) => Err(format!(
                        "{} not yet implemented for -=({:?} and {:?})=-",
                        op,
                        x.as_literal_type(),
                        y.as_literal_type()
                    )),
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Expr::*;
//     use super::LiteralVal::*;
//     use super::*;
//     use crate::TokenType;

//     #[test]
//     fn print_out() {
//         let plus = Token {
//             token_type: TokenType::Plus,
//             lexeme: "+".to_string(),
//             literal: None,
//             line_num: 0,
//         };
//         let star = Token {
//             token_type: TokenType::Star,
//             lexeme: "*".to_string(),
//             literal: None,
//             line_num: 0,
//         };
//         let lit_test = Literal { val: NumVal(326.5) };
//         let g = Grouping {
//             expr: Box::from(Literal { val: NumVal(23.5) }),
//         };
//         let tree_time = Binary {
//             op: star,
//             l: Box::from(Unary {
//                 op: plus,
//                 r: Box::from(lit_test),
//             }),
//             r: Box::from(g),
//         };
//         tree_time.pretty_print();
//     }
// }

// // NOTE: This approach only works better for the Java implementation, come back to if needed
// // pub struct GenerateAst;
// // impl GenerateAst {
// //     fn main() -> Result<(), String> {
// //         let args: Vec<String> = env::args().collect();
// //         if args.len() != 2 {
// //             println!("Usage: generate_ast <output dir>");
// //             exit(64);
// //         }
// //         let output_dir: &str = &args[1];
// //         GenerateAst::define_ast(output_dir, "Expr", vec![
// //             "Binary   : Expr left, Token operator, Expr right",
// //             "Grouping : Expr expression",
// //             "Literal  : Object value",
// //             "Unary    : Token operator, Expr right",
// //         ])?;
// //         Ok(())
// //     }

// //     fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> Result<(), String> {
// //         let path: String = format!("{}/{}.rs", output_dir, base_name);
// //         let file = match File::create(&path) {
// //             Ok(file) => file,
// //             Err(err) => return Err(format!("Failed to create file: {}", err)),
// //         };

// //         // Create a buffered writer to write to the file
// //         let mut writer = BufWriter::new(file);
// //         writer.write("");
// //         Ok(())
// //     }
// // }
