extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use regex::Regex;

pub struct Config {
    pattern: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("wrong number of arguments (2 needed)")
        }
        let pattern = args[1].clone();
        let filename = args[2].clone();
        // if "CASE_INSENSITIVE" env variable doesn't exist is_err() returns true
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { pattern, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("searching: {}", config.pattern);
    println!("in file: {}", config.filename);

    let mut f = File::open(&config.filename)?;
    let mut text = String::new();
    f.read_to_string(&mut text);
    println!("content:\n{}", text);
    
    search_by_case(config, &text);
    Ok(())

}

fn search_by_case(config: Config, text: &str) -> bool {
    let pattern = &config.pattern;

    if config.case_sensitive {
        search(pattern, &text)
    }
    else { search_case_insensitive(pattern, &text)}
}

fn search(pattern: &str, text: &str) ->  bool {
    let pattern = Regex::new(&pattern).unwrap();
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
   return search(&insensitive_pattern, text)
}


#[cfg(test)]
mod test {

    use super::*;
    use std::process::Command;

    #[test]
    fn test_search() {
        let found = search(
            "fast,\\s+\\w+",
            "Rust: safe, fast, productive. Pick three."
        );
        assert!(found, "pattern not found!")
    }

    #[test]
    fn neg_test_search() {
        let found = search(
            "Java",
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


    #[test]
    fn test_env_variable() {
        env::set_var("CASE_INSENSITIVE", "true");

        let mut args = Vec::new();
        args.push(String::from("mock_binary_arg"));
        args.push(String::from("love"));
        args.push(String::from("mock_file_path"));
        let conf = Config::new(&args).unwrap();

        let found = search_by_case(conf, "Love in all the wrong places");
        assert!(found, "search was not case insensitive, environment var failed");

        env::remove_var("CASE_INSENSITIVE");

        let conf = Config::new(&args).unwrap();

        let found = search_by_case(conf, "Love in all the wrong places");
        assert!(!found, "search was case insensitive, even though env var doesn't exist");

    }

}
