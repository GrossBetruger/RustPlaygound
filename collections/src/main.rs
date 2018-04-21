//#![feature(iterator_step_by)]

use std::collections::HashMap;


#[derive(Debug)]
enum SpreedSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("empty vec {:?}", v);

    let v2 = vec![1, 1, 2]; // creates a Vec<integer>
//    v2.connect(); // doesn't compile but will expose the type of v2 in the compiler error message

    v.push(1);
    v.push(2);

    let unsafe_index = 100;

    println!("{:?} is what you get", v.get(unsafe_index)); // safely compiles
//    v[unsafe_index]; // panics, for obvious reasons

    for i in 0..100 {
        let x = v.get(i);
        match x {
            Some(i32) => println!("found a value: '{:?}'", x),
            None => () // if there's no integer do nothing
        }
    };

    for i in 0..5 {
        let x = v.get(i);
        let default: i32 = 77;
        println!("found value: {}", x.unwrap_or(&default));
    }

//    let n = v.get(0); // this will break as we try to push more elems to v because it's an immutable borrow
    { let n = v.get(0); } // this doesn't break because n goes out of scope
    v.push(9);

//    simple iteration over a vector
    for i in &v {
        println!("i: {}", i);
    }

    v.push(10);

//    iterate and mutate
    for i in &mut v {
        *i += 1;
    }

    for i in &v {
        println!("i after mutation: {}", i);
    }

    let row = vec![SpreedSheetCell::Int(3), SpreedSheetCell::Float(2.718),
               SpreedSheetCell::Text(String::from("Alfred"))]; // vec with different types using enum

    println!("spreedsheet row: {:?}", row);

    println!("vec before popping {:?}", v);
    println!("popped top: {}", v.pop().unwrap_or(-1));
    println!("vec after popping {:?}", v);

//    STRINGS

    let mut data = "some literal";

    let mut s = data.to_string();

//    data.push_str("more literals"); // won't compile - literals are &str, and slices are immutable
    s.push('\x20'); // bytes literals! push for chars, push_str for slices...
    s.push_str("more literacy!"); // append a slice to a String
    println!("{}", s);

//    UTF encoded literals
    let konnichiwa = String::from("こんにちは");
    println!("{} y'all", konnichiwa);

    for c in konnichiwa.into_bytes() {
        println!("char: {} ord: {}", c as char, c);
    }
    println!();

    for c in s[..4].to_string().into_bytes() {
        println!("char: {} ord: {}", c as char, c);
    }

    let lexem1 = String::from("fast");
    let lexem2 = String::from("er");
    println!("add {}!", lexem1 + &lexem2); // lexem1 has been moved and can no longer be used
//    println!("{}", lexem1); // won't compile

    let mut s1 = String::from("tic");
    let mut s2 = String::from("tac");
    let mut s3 = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}", s1, s2, s3); // doesn't take ownership of any string!
    s1.push('\x61'); // still valid
    println!("{}", tic_tac_toe);

    let zdrustvite = String::from("Здравствуйте");
    println!("you had me at: {}", zdrustvite.len()); // two bytes(u8) per char

    for i in 0..zdrustvite.len() -1 {
        if i % 2 == 0 {
            // a Cyrillic char is encoded in every two bytes, those can be slices to enable charAt behaviour
            println!("{}", &zdrustvite[i..i+2]);
        }
    }

    let naruste = String::from("नमस्ते");
    println!("my bytes ({}) acknowledge your bytes", naruste.len());

    for i in 0..naruste.len() -1 {
        if i % 3 == 0 {
            println!("{}", &naruste[i..i + 3]) // Hindi char is encoded in 3 bytes,
            // some chars are diacritic and they are merged with other chars to encode a grapheme cluster
            // the string contains six chars (two diacritic and four regular) and four grapheme clusters
        }
    }
    println!("{}", naruste);

//    a simpler approach to accessing chars would be
    for c in naruste.chars() {
        println!("{}", c);
    }

//    there is no way to access grapheme clusters in the standard lib

//    HASHMAPS

    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 2);
    scores.insert(String::from("Blue"), 10);

//    let mut empty_hash_map = HashMap::new(); // won't compile - compiler can't guess type because there's no assignment
    let mut empty_hash_map: HashMap<String, i32> = HashMap::new();

//    now a less lame instantiation

    let teams = vec!["Red", "Blue"];
    let initial_scores = vec![2, 10];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("Red score: {:?}", scores.get(&"Red").unwrap_or(&&-1)); // value is Some(&2), so we need a clumsy unwrap

//    iterating over a hashmap
    for (key, val) in &scores {
        println!("key: {}, value: {}", key, val);
    }
}