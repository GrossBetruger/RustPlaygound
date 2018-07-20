extern crate add_one;
extern crate add_two;

fn main() {
    println!("6 + 1 is: {}", add_one::add_one(6));
    println!("5 + 2 is: {}", add_two::add_two(5));

}
