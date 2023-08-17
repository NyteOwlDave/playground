
#![allow(dead_code)]

use std::process;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use super::environ;

pub fn poem() {
    // dave::hello();
    // ip::ip_test();
    // rect::rect_test();
    // coin::coin_slot(Coin::Quarter);
    let config = Config::new().unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(err) = run(config) {
        println!("File I/O error: {}", err);
        process::exit(2);
    };
}

#[derive(Debug)]
struct Config {
    filename: String,
    pattern: String,
    case_insensitive: bool,
}

impl Config {
    fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("Too few arguments")
        }
        let pattern = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = match environ::getenv("NOCASE") {
            Ok(_) => true,
            _ => false,
        };
        Ok(Config {
            pattern,
            filename,
            case_insensitive,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let lines = if config.case_insensitive {
        println!("case insensitive");
        search_case_insensitive(&config.pattern, &content)
    } else {
        println!("case sensitive");
        search(&config.pattern, &content)
    };
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search() {
        let query = "duct";
        let content = "\
Rust
safe, fast, productive
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive"],
            search(query, content)
        );
    }
}
