extern crate oop;

use oop::*;

fn main() {

    let mut avg_lst  = AveragedCollection::new();

    avg_lst.add(3.);
    avg_lst.add(2.);
    // avg_lst.average = 7.; // won't compile average is encapsulated
    println!("average: '{}'", avg_lst.average());

}

