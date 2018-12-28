


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
    

}
