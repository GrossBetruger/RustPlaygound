fn main() {
    let guess: u32 = "42".parse().expect("not a number");
    println!("{}", guess);

    let hex1: u32 = 0x20; // 32
    let hex2 = 0x101u8; // overflow on unsigned byte - value 1
    let bin = 0b10000; // 16

    println!("hex1, {}", hex1);
    println!("hex2, {}", hex2);
    println!("bin, {}", bin);

    let quotient = 3.1 / 2.;
    let int_quotient = 3 / 2;

    println!("quotient, {}", quotient);
    println!("int_quotient, {}", int_quotient);

    let remainder = 43 % 5;

    // bools

    println!("43 modulo 5, {}", remainder);

    let pred: bool = false;
    if !(pred){
        println!("pred, {}", pred);
    }

    let two_face = if pred {0} else {1};

    println!("two face, {}", two_face);

    // chars

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let floral_heart = '\u{2766}';
    println!("heart eyed cat, {}", heart_eyed_cat);
    println!("floral heart, {}", floral_heart);
    println!("{}", '❤'.escape_unicode());
    println!("{}", '\u{2764}');

    // tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("tup[0], {}", x);
    println!("tup[-1], {}", z);


    println!("tup[1], {}", tup.1);

    // arrays

    let array = [1, 1, 2, 3, 5, 8];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let first_fib = array[0];
    let last_fib = array[array.len() -1];

    println!("first fib, {}", first_fib);
    println!("last fib, {}", last_fib);




}
