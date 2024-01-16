use std::env;
use std::process;
use rustgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
	println!("Problem in parsing arguments: {e}");
	process::exit(1);  // Exiting the program with error
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = rustgrep::run(config) {
	println!("Application error: {e}");
	process::exit(1);  // Exiting the program with error
    }
}

