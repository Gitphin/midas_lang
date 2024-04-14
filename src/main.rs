use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

fn run_prompt() -> Result<(), io::Error> {
    // loooop 
    loop {
        print!("> ");
        // check if can properly display
        match io::stdout().flush() {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        let stdin = io::stdin();
        let mut h = stdin.lock();
        let mut buff = String::new();
        match h.read_line(&mut buff) {
            Ok(c) => {
                if c <= 1 {
                    return Ok(());
                }
            }
            Err(err) => return Err(err)
        }
        println!("Output: {}", buff);
    }
}

fn run_file(path: &str) -> Result<(), io::Error> {
    // reads file contents and runs
    match fs::read_to_string(path) {
        Ok(contents) => {
            run(&contents);
            Ok(())
        },
        Err(err) => Err(err),
    }
}

fn run(_s: &str) -> Result<(), String> {
    Err("Not done yet :v)".to_string())
}

// fn scan_tokens() {
//     not_implemented()
// }

fn main() -> Result<(), io::Error> {
    // get all args, match for how many given
    let args: Vec<String> = env::args().collect();   
    match args.len() {
        // 2 args = script file, 1 = interactive mode, else err
        2 => run_file(&args[1]),
        1 => Ok(run_prompt()?),
        _ => {
            println!("Usage: midasgo[script]");
            std::process::exit(64);
        }
    }
}
