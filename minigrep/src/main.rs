extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use regex::Regex;

struct Config {
    pattern: String,
    filename: String
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("wrong number of arguments (2 needed)")
        }
        let pattern = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { pattern, filename })
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    println!("searching: {}", config.pattern);
    println!("in file: {}", config.filename);

    let mut f = File::open(&config.filename).expect(
        &format!("couldn't find file: '{}'", &config.filename)
    );
    let mut text = String::new();
    f.read_to_string(&mut text);
    println!("content:\n{}", text);

    let pattern = Regex::new(&config.pattern).unwrap();

    println!();
    if pattern.is_match(&text) {
        println!("pattern found! '{}'", &pattern);
        for cap in pattern.captures_iter(&text) {
            println!("match: {}", &cap[0]);
        }
    }

    else { println!("pattern not found! '{}'", &pattern) }
    Ok(())

}

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config = Config::new(&args).unwrap(); // parse_cli
    run(config);

}
