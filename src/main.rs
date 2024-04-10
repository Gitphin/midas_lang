use std::env;
use std::fs;
use std::process::exit;


fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
    // let contents = fs::read_to_string(path).expect("Couldnt read file");
    // run(contents)
}

fn run_prompt() {

}

fn run(_contents: &str) -> Result<(), String> {
    return Err("Not done".to_string());
}

fn main() {
    let args: Vec<String> = env::args().collect();   

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        run_prompt();
    }
}
