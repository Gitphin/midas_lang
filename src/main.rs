use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use colored::Colorize;
use rand::Rng;

// Terminal view, takes in user input
fn run_prompt() -> Result<(), io::Error> {
    // loooop 
    loop {
        print!("{}", "Midas > ".truecolor(255,210,0).bold());
        // check if can properly display
        match io::stdout().flush() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        // get user input
        let stdin = io::stdin();
        let mut h = stdin.lock();
        let mut buff = String::new();
        h.read_line(&mut buff)?;
        // println!("\nOutput: {}", buff);
        match buff.trim() {
            "quit" | "q" => {println!("Goodbye o7"); break Ok(())},
            "rng" => {let mut rng = rand::thread_rng(); println!("{} is your random number!", rng.gen_range(1..101).to_string().bold())},
            "hello" => println!("hi :v)"),
            _  => (),
        }
    }
}
 // Reads file contents and runs
fn run_file(path: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(path)?;
    // TO FIX: let _ 
    let _ = run(&contents);
    Ok(())
}
// Not implemented yet, should run this
fn run(_s: &str) -> Result<(), String> {
    Err("Not done yet :v)".to_string())
}

// fn scan_tokens() {
//     not_implemented()
// }

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();   
    match args.len() {
        // 2 args = script file, 1 = interactive mode, else exit
        2 => run_file(&args[1]),
        1 => Ok(run_prompt()?),
        _ => {
            println!("Usage: midasgo[script]");
            std::process::exit(64);
        }
    }
}
