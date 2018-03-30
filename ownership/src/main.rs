fn main() {
    { // s is not valid here

        let s = "hello";
//        do stuff with s
    } // the scope is over s is no longer valid

    let mut s1 = String::from("hello");
    s1.push_str(", world!");

    let mut literal = "mut";

//    literal.push_str("able"); // not legal - literal strings are immutable

    println!("{}", s1);


//    two ints are pushed to the stack x=5, y=5
    let x = 5;
    let y = x;


//    s1 and s2 point to the same buffer, (len, and capacities moved from s1 to s2)
    let s1 = String::from("hello");
    let s2 = s1;

//    println!("{}", s1); // will raise compiler error - because s1 was moved to s2 and invalidated
//    to avoid double-free errors when they both get out of scope

    let s1 = String::from("original"); // shadowing previous s1
    let s2 = s1.clone();
//  this time s1 is not invalidated because clone() created a deep copy of s1
    println!("s1: {}, s2: {}", s1, s2);


    let x = 5;
    let y = x;
//  valid without clone, because primitives are copied by default (implementing the Copy trait)
    println!("x = {}, y = {}", x, y);
}
