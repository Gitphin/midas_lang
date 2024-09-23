use crate::scanner;
use crate::scanner::{Token, TokenType};
// use std::process::exit;
// use std::env;
// use std::fs::File;
// use std::io::{BufWriter, Write};

// Need this to handle literal values within AST tree, similar to scanner impl
pub enum LiteralVal {
    NumVal(f32),
    StringVal(String),
    TrueVal,
    FalseVal,
    NullVal,
}

fn unwrap_as_f32(literal: Option<scanner::LiteralVal>) -> f32 {
    match literal {
        Some(scanner::LiteralVal::IntVal(i)) => i as f32,
        Some(scanner::LiteralVal::FVal(i)) => i as f32,
        _ => panic!("Could not unwrap"),
    }
}

fn unwrap_as_str(literal: Option<scanner::LiteralVal>) -> String {
    match literal {
        Some(scanner::LiteralVal::StringVal(s)) => s.clone(),
        Some(scanner::LiteralVal::IdentifierVal(s)) => s.clone(),
        _ => panic!("Could not unwrap"),
    }
}
impl LiteralVal {
    pub fn format_str(&self) -> String {
        match self {
            LiteralVal::NumVal(v) => v.to_string(),
            LiteralVal::StringVal(v) => (&v).to_string(),
            LiteralVal::TrueVal => "true".to_string(),
            LiteralVal::FalseVal => "false".to_string(),
            LiteralVal::NullVal => "null".to_string(),
        }
    }
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
}
// AST expression implementation
pub enum Expr {
    Binary {
        op: Token,
        l: Box<Expr>,
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
// Expression to
impl Expr {
    // Matches expr and formats to print correctly
    pub fn format_str(&self) -> String {
        match self {
            Expr::Binary { l, r, op } => {
                format!("({} {} {})", op.lexeme, l.format_str(), r.format_str())
            }
            Expr::Grouping { expr } => format!("(group {})", (*expr).format_str()),
            Expr::Literal { val } => format!("{}", val.format_str()),
            Expr::Unary { op, r } => format!("({} {})", &op.lexeme, (*r).format_str()),
        }
    }
    // Prints structure of syntax tree (useful for debugging)
    pub fn pretty_print(&self) {
        println!("{}", self.format_str());
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

// NOTE: This approach only works better for the Java implementation, come back to if needed
// pub struct GenerateAst;
// impl GenerateAst {
//     fn main() -> Result<(), String> {
//         let args: Vec<String> = env::args().collect();
//         if args.len() != 2 {
//             println!("Usage: generate_ast <output dir>");
//             exit(64);
//         }
//         let output_dir: &str = &args[1];
//         GenerateAst::define_ast(output_dir, "Expr", vec![
//             "Binary   : Expr left, Token operator, Expr right",
//             "Grouping : Expr expression",
//             "Literal  : Object value",
//             "Unary    : Token operator, Expr right",
//         ])?;
//         Ok(())
//     }

//     fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> Result<(), String> {
//         let path: String = format!("{}/{}.rs", output_dir, base_name);
//         let file = match File::create(&path) {
//             Ok(file) => file,
//             Err(err) => return Err(format!("Failed to create file: {}", err)),
//         };

//         // Create a buffered writer to write to the file
//         let mut writer = BufWriter::new(file);
//         writer.write("");
//         Ok(())
//     }
// }
