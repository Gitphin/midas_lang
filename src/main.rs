#![allow(dead_code)]
#![allow(unused_variables)]
mod expr;
mod interpreter;
mod literals;
mod parser;
mod scanner;
mod statement;
use crate::interpreter::*;
use crate::parser::*;
use crate::scanner::*;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

// Terminal view, takes in user input
fn run_prompt() -> Result<(), String> {
    // NOTE: this might need to be put in loop, will see!
    let mut intr: Interpreter = Interpreter::new();
    let mut buff = String::new();
    loop {
        print!("{}", "(/•ิ_•ิ)/ → ");
        // check if can properly display
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        // get user input
        let stdin = io::stdin();
        let mut h = stdin.lock();
        match h.read_line(&mut buff) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        // quit loop
        if buff.trim() == "exit" {
            println!("{}", "Stay gold, Ponyboy...");
            break Ok(());
        }
        // run user input
        match run(&mut intr, &buff) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
        buff.clear();
    }
}
// Reads file contents and runs
fn run_file(path: &str) -> Result<(), String> {
    let mut intr = Interpreter::new();
    match fs::read_to_string(path) {
        Ok(c) => return run(&mut intr, &c),
        Err(e) => return Err(e.to_string()),
    }
}
// Run and get tokens
fn run(intr: &mut Interpreter, contents: &str) -> Result<(), String> {
    let mut s = Scanner::new(contents);
    let tokens = s.scan_tokens()?;
    let mut p = Parser::new(tokens);
    let stmnts = p.parse()?;
    for st in stmnts {
        intr.interpret_statement(st)?;
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // 2 args = script file, 1 = interactive mode, else exit
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: midas_lang[script]");
            exit(64)
        }
    }
}
