//#![feature(iterator_step_by)]

extern crate reduce;

use reduce::Reduce;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::collections::hash_map::Entry;

#[derive(Debug)]
enum SpreedSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn mean(list: Vec<i32>) -> f32 {
    let list_length = list.len();
    list.into_iter().reduce(|a, b| a + b).unwrap_or(0) as f32 / list_length as f32
}

fn median(list: Vec<i32>) -> f32 {
    let length = list.len();
    match length % 2 == 0 {

        false => {
                let idx = ((length / 2) as f32).ceil() as usize;
                list[idx] as f32
                },

         true => {
                 let idx1 = ((length / 2) as f32).floor() as usize -1;
                 let idx2 = idx1 + 1;
                 (list[idx1] + list[idx2]) as f32 / 2.
                 }
    }
}

fn mode(list: Vec<i32>) -> i32 {
    let max = 0;
    let mode = 0;

    let mut counter = HashMap::new();
    for n in list.iter() {
        *counter.entry(n).or_insert(0) += 1;
    }

    let mut counter_vec: Vec<_> = counter.into_iter().collect();
    counter_vec.sort_by(|a, b| b.1.cmp(&a.1));
    return *counter_vec[0].0;
}

fn append_hay(s: &mut String) -> &mut String {
    s.push_str("hay");
    s
}

fn append_ay(s: &mut String) -> &String {
    s.push_str("ay");
    s
}

fn rotate_first(s: String) -> String{
    let first = s.chars().take(1).collect::<String>();
    let mut tail = s.chars().skip(1).collect::<String>();
    tail.push_str(&first);
    tail
}

fn first_is_vowel(input: &String) -> bool {
    let first = input.to_lowercase().chars().next().unwrap();
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    vowels.iter().any(|v| first == *v)
}

fn pig_latin(input: &mut String)  -> &str{
    match first_is_vowel(input) {
        true => { append_hay(input)},
        false => {"yo"}
    }
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

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
    println!("Red score: {:?}", scores.get(&"Red").unwrap_or(&&-1)); // value is Some(&2), so we need a clumsy unwrap

    scores.insert(&"Blue", &11);
//    iterating over a hashmap
    for (key, val) in &scores {
        println!("key: {}, value: {}", key, val);
    }

//    insert only if key doesn't exist using the 'entry' API

    scores.entry(&"Blue").or_insert(&100);
    scores.entry(&"Purple").or_insert(&200);
    println!();
    for (k, v) in &scores {
        println!("key: {}, value: {}", k, v);
    }

//    count words in text

    let mut f = File::open(&"rust_book_chapter.txt").expect("failed to open file");
    let mut text = String::new();

    f.read_to_string(&mut text).expect("couldn't read file to string");

    let mut word_counter = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_counter.entry(word).or_insert(0);
        *count += 1;
    }

    println!();

    for (k, v) in &word_counter {
        if *v < 20 {
            continue;
        }
        println!("{} -> count: {}", k, v);
    }

//    EXERCISES
//    1
//    Given a list of integers, return the mean, median , and mode.

    let l: Vec<i32> = vec![1, 1, 2, 4, 5];
    let l_odd = l.clone();
    let l_even: Vec<i32> = vec![1, 1, 2, 4];

    println!("mean: {}", mean(l));
    println!("median even: {}", median(l_even));
    println!("median odd: {}", median(l_odd));

    let obivious_mode = vec![2, 6, 6, 77, 77, 77, 2, 0];
    println!("and the mode is: {}", mode(obivious_mode));

//    EXERCISES
//    2
//    Given a list of integers, return the mean, median , and mode.

    println!("hello rotate first: {}", rotate_first(String::from("hello")));
    println!("नमस्ते rotate first: {}", rotate_first(String::from("नमस्ते")));
    let mut ola = String::from("Ola");
//    pig_latin(String::from("hello"));
//    pig_latin(String::from("Ola"));
//    pig_latin(ola);

    println!("'Ola' in pig latin: '{}'", pig_latin(&mut ola));
    println!("'aloha' in pig latin: '{}'", pig_latin(&mut String::from("aloha")));
//    pig_latin(String::from("f do"));
//    println!("'hello' first is vowel: {}", pig_latin(String::from("नमस्ते")));

}