extern crate regex;
extern crate minigrep;

use std::env;
use std::process;

use minigrep::*;

fn main() {

    let config = Config::new(env::args()).unwrap(); // parse_cli
    if let Err(e) = run(config) {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
