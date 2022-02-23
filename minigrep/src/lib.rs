use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let filename = args[2].clone();    
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
} 

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut matches = Vec::new();   
    for elem in contents.lines() {
        if case_sensitive == false && elem.to_lowercase().contains(&query.to_lowercase()) {
            matches.push(elem);
        } else if elem.contains(&query) {
            matches.push(elem);
        }                
    };

    return matches;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents, false));
    }

    #[test]
    fn case_sensitive() {
        let query = "DUCT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let empty_vec: Vec<&str> = Vec::new();

    assert_eq!(empty_vec, search(query, contents, false));
    }

    #[test]
    fn case_insensitive() {
        let query = "DUCT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents, true));
    }
}
