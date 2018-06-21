extern crate colored;
extern crate regex;

use colored::*;
use regex::Regex;


fn get_offset(re: Regex, input: &str) -> (usize, usize) {
    let re_match = re.find(input).unwrap();
    (re_match.start(), re_match.end())
}


fn paint_match(re: Regex, input: &str) {
    let (start, end) = get_offset(re, input);
    let mut head = &input[0..start];
    let colored = &input[start..end].red();
    println!("{}{}", head, colored);
}

fn main() {

    let mut input = "123boom";
    let re = Regex::new("[a-z]+").unwrap();
    paint_match(re, input);

}