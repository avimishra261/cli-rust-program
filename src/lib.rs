use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub sensitive_case: bool,
}

impl Config{
    pub fn build(
	mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
	args.next();

	let query = match args.next() {
	    Some(arg) => arg,
	    None => return Err("Didn't get a query string"),
	};

	let file_path = match args.next() {
	    Some(arg) => arg,
	    None => return Err("Didn't get a file path"),
	};

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
    contents.lines().filter(|line| line.contains(query)).collect()
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

