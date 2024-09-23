mod scanner;
mod expr;
mod parser;
use crate::parser::*; 
use crate::scanner::*;
use std::process::exit;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

// Terminal view, takes in user input
fn run_prompt() -> Result<(), String> {
    // NOTE: this might need to be put in loop, will see!
    let mut buff = String::new();
    loop {
        print!("{}","(/•ิ_•ิ)/ → ");
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
        match run(&buff) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
        buff.clear();
    }
}
// Reads file contents and runs
fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Ok(c) => return run(&c),
        Err(e) => return Err(e.to_string())
    }
}
// Run and get tokens
fn run(contents: &str) -> Result<(), String> {
    let mut s = Scanner::new(contents);
    let tokens = s.scan_tokens()?;
    let mut p = Parser::new(tokens);
    let e = p.parse()?;
    let r = e.eval()?;
    println!("{}", r.format_str());
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
