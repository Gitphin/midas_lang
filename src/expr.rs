#![allow(dead_code)]
#![allow(unused_variables)]

use crate::literals::LiteralVal;
use crate::literals::LiteralVal::*;
use crate::enviro::Enviro;
use crate::scanner::{Token, TokenType};

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
    Variable {
        name: Token,
    }
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
            Expr::Variable { name } => format!("var {}", name.lexeme),
        }
    }
    // Prints structure of syntax tree (useful for debugging)
    pub fn pretty_print(&self) {
        println!("{}", self.format_str());
    }
    // This acts as my interpeter, evaluates expressions
    pub fn eval(&self, enviro: &Enviro) -> Result<LiteralVal, String> {
        match self {
            Expr::Variable { name } => match enviro.get(&name.lexeme) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("Variable -=({})=- has not been declared!", name.lexeme))
            },
            Expr::Literal { val } => Ok(val.clone()),
            Expr::Grouping { expr } => expr.eval(enviro),
            Expr::Unary { op, r } => {
                let r = r.eval(enviro)?;
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
                let l = l.eval(enviro)?;
                let r = r.eval(enviro)?;
                match (l.clone(), op.token_type, r.clone()) {
                    (NumVal(x), TokenType::Plus, NumVal(y)) => Ok(NumVal(x + y)),
                    (NumVal(x), TokenType::Minus, NumVal(y)) => Ok(NumVal(x - y)),
                    (NumVal(x), TokenType::Slash, NumVal(y)) => {
                        if y != 0.0 {
                            Ok(NumVal(x / y))
                        } else {
                            Err(format!(
                                "Cannot divide -=({})=- by 0, results in infinity",
                                x
                            ))
                        }
                    }
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
                    // UNCOMMENT IF YOU WANT TO TREAT NUM + STR OPERATIONS AS VALID

                    //                     (NumVal(x), TokenType::Plus, StringVal(s)) => {
                    //                         let x2 = x.to_string();
                    //                         Ok(StringVal(format!("{}{}", x2, s)))
                    //                     }
                    //                     (StringVal(s), TokenType::Plus, NumVal(x)) => {
                    //                         let x2 = x.to_string();
                    //                         Ok(StringVal(format!("{}{}", x2, s)))
                    //                     }
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
