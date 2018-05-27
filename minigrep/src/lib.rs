extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use regex::Regex;

pub struct Config {
    pattern: String,
    filename: String
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("wrong number of arguments (2 needed)")
        }
        let pattern = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { pattern, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("searching: {}", config.pattern);
    println!("in file: {}", config.filename);

    let mut f = File::open(&config.filename)?;
    let mut text = String::new();
    f.read_to_string(&mut text);
    println!("content:\n{}", text);

    let pattern = Regex::new(&config.pattern)?;

    search(pattern, &text);
    Ok(())

}

fn search(pattern: Regex, text: &str) ->  bool {
    println!();
    let found = pattern.is_match(text);
    if found {
        println!("pattern found! '{}'", pattern);
        for cap in pattern.captures_iter(text) {
            println!("match: {}", &cap[0]);
        }
    } else { println!("pattern not found! '{}'", &pattern) }
    found
}

fn search_case_insensitive(pattern: &str, text: &str) -> bool {
   let mut insensitive_pattern = String::from("(?i)");
   insensitive_pattern.push_str(&pattern);
   return search(Regex::new(&insensitive_pattern).unwrap(), text)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_search() {
        let found = search(
            Regex::new("fast,\\s+\\w+").unwrap(),
            "Rust: safe, fast, productive. Pick three."
        );
        assert!(found, "pattern not found!")
    }

    #[test]
    fn neg_test_search() {
        let found = search(
            Regex::new("Java").unwrap(),
            "Rust: safe, fast, productive. Pick three."
        );
        assert!(!found, "non existing pattern was found!")
    }

       #[test]
    fn test_search_case_insensitive() {
        let found = search_case_insensitive(
            "Safe",
            "Rust: safe, fast, productive. Pick three."
        );
        assert!(found, "pattern was not found!")
    }
}