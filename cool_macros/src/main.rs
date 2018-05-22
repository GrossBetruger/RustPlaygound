use std::*;

fn main() {
    // injects the text in the file into a &static str in the compiled bin
    let my_str = include_str!("spanish.txt");
    assert_eq!(my_str, "adiós\n");
    print!("{}", my_str);
}
