use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub sensitive_case: bool,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
	if args.len() < 3{
	    return Err("not enough arguments");
	}
	
	let query = args[1].clone();
	let file_path = args[2].clone();

	let sensitive_case = env::var("SENSITIVE").is_ok();


	Ok(
	Config {query,file_path,sensitive_case}
	)  // Returning struct in Result can show a user friendly error msg on the terminal 
    }
}

// Run function
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.sensitive_case {
	search_insensitive(&config.query, &contents)
    } else {
	search(&config.query, &contents)
    };

    for line in results {
	println!("{line}");
    }

    Ok(())
}

// Searching query in filepath functions //

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
	if line.contains(query) {
	    results.push(line);
	}
    }
    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
	if line.to_lowercase().contains(&query) {
	    results.push(line);
	}
    }

    results
}

