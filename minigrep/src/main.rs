extern crate regex;
extern crate minigrep;

use std::env;
use std::process;

use minigrep::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config = Config::new(&args).unwrap(); // parse_cli
    if let Err(e) = run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }
}
