

fn print_point(&(x, y): &(i32, i32)) { // the parameter is a pattern!
    println!("x coord: '{}'", x);
    println!("y coord: '{}'", y);
}


fn main() {

    let favorite_color: Option<&str> = None;

    match favorite_color {
        Some(color) => println!("choosing your fav color: '{}'", color),
        _ => println!("fav color wasn't specified")
    }

    let age : Result<u8, _> = "34".parse();

    match age {
        Ok(some_age) => println!("your age is: '{}'", some_age),
        Err(err) => println!("failed to parse age... reason : {:?}", err)
    }
    
    let mut stack = Vec::new();

    for i in 1..7 {
        stack.push(i);
    }

    while let Some(top) = stack.pop() {
        println!("TOS popped: '{}'", top);
    }

    let nums = 0..5;

    for (i, num) in nums.rev().enumerate() {
        println!("index: '{}', num: '{}'", i, num)
    }

    // pattern in let statement
    let (pi, e) = (3.1415, 2.718);

    println!("constants: {} {}", pi, e);


    let point: (i32, i32) = (11, -11);
    print_point(&point);

    // refutability

//    let Some(x) = Some(3); // won't compile because 'let' requires an irrefutable pattern

     if let Some(_x) = Some(3) {
         println!("this definitely compile because 'if let' requires a refutable pattern")
     }

//    if let x = 44 { } // no way... the pattern '44' is irrefutable

}
