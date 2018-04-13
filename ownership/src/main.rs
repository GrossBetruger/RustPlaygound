fn takes_ownership(s: String){
    println!("I own: {}", s);
}


fn copies(x: i32){
    println!("copied {}, and printed it proudly", x);
}


fn gives_ownership() -> String {
    let ret_val = String::from("returned value");
    ret_val
}


fn calc_length(str: &String) -> usize {
    str.len()
}


fn changer(str: &String) {
//    str.push_str("suffix"); // doesn't compile cannot changed str because it's borrowed
}


fn legal_changer(str: &mut String){
    str.push_str("suffix")
}

//fn dangle(unused_param: &String) -> &String{
//    let in_scope_string = String::from("use me after free!");
//    &in_scope_string
//}


fn no_dangle() -> String{
    let ad_hoc = String::from("out of thin air");
    ad_hoc
}

fn takes_and_gives_back(s: String) -> String {
    s
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn safe_first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}

fn safe_first_word_better_signature(s: &str) -> &str{
//    this is a better function signature than "safe_first_word"'s because it enables taking both
//    &String and &str as arguments
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}

fn main() {
    {
        // s is not valid here

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

    let argument = String::from("arg string");

    takes_ownership(argument);
    //    function takes ownership of argument, var argument is invalidated
    //    println!("printing the invalid: {}", argument); // will cause a 'use of moved value err'

    let scalar = 22;

    copies(scalar);

    println!("what do you know, {} is still valid!", scalar);


    let yet_another_str = gives_ownership();

    let pushed_around = takes_and_gives_back(yet_another_str);

    println!("still valid in this scope: {}", pushed_around);

    //    println!("not so valid anymore: {}", yet_another_str); // the compiler's not gonna like this one

    let lengthy = String::from("empty I am not");

    println!("the length of '{}' is: {}", lengthy, calc_length(&lengthy));

    let mut borrowed = String::from("borrow me!");
    //    changer(&borrowed); // doesn't compile changer cannot change a borrowed object
    legal_changer(&mut borrowed);
    println!("'{}' was changed using a mutable reference", borrowed);

    let mut double_borrow = String::from("borrow me twice shame on you!");

    let s1 = &mut double_borrow;
    //    let s2 = &mut double_borrow; // doesn't compile due to: second mutable borrow!

    let mut contex_borrow = String::from("I'll be yours for the whole block...");

    {
        let r1 = &mut contex_borrow;
    }

    let mut r2 = &mut contex_borrow; // compiles with joy! first borrower went out of scope


    let mut immutable_borrow = String::from("promise not to break me");

    let r1 = &immutable_borrow; // all good
    let r2 = &immutable_borrow; // nothing happens
    //    let r3 = &mut immutable_borrow; // all hell breaks loose, can't mix mut and immut borrows


    //    Rust protects from use-after-free vulns by detecting dangle pointers in compile-time
    //    let reference_to_nothing = dangle(& String::from("unused arg")); // will not compile -
    //    returning a reference to an already dropped value

    let funcy_string = no_dangle(); // no problem, the String is created in the function and
    // passed to this scope

    let mut two_words = String::from("two words");
    println!("first words end at index: {}", first_word(&two_words));

    let first_word_indx = first_word(&two_words);

    two_words.clear();
    //    two_words changed but the alleged index now points to an invalid location

    //    Slices!!
    let pizza = String::from("slice me");
    let first_slice_idx = first_word(&pizza);
    let first_slice = &pizza[0..first_slice_idx];
    let second_slice = &pizza[first_slice_idx + 1..pizza.len()];

    println!("slices of '{}', 1: '{}', 2: '{}'", pizza, first_slice, second_slice);

    let some_str = String::from("a bunch of chars");
    let slice1 = &some_str[0..3];
    let slice2 = &some_str[..3];
    println!("slices are equal this is {}", slice1 == slice2);

    let mut two_words = String::from("first last");
    let safe_first = safe_first_word(&two_words);
    println!("safely extracted '{}' from '{}'", safe_first, two_words);

//    two_words.clear(); // this will not compile because "safe_first_word" created a mutable borrow
    let first = safe_first_word_better_signature("literals are &str");
//    let first = safe_first_word(&"literals are &str"); // won't compile

    let a = [1, 1, 2, 3, 5, 8];
    println!("array slice: {:?}", &a[2..4]);
}