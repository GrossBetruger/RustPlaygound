extern crate regex;

use regex::Regex;


fn main() {

    //https://stackoverflow.com/questions/47298336/case-insensitive-string-matching-in-rust/50271550#50271550
    let re = Regex::new(r"(?i)μτς").unwrap();
    let mat = re.find("ΜΤΣ").unwrap();
    println!("{:?}", mat);
}
