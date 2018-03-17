// multiline comments
// are kind of boring
// in Rust

fn val_printer(val: i32) {
    println!("the argument value is: {}", val);
}


fn two_val_printer(val1: i32, val2: i32) {
    println!("val1 = {}", val1);
    println!("val2 = {}", val2);
}


fn three() -> u8 {
    // final expression returns 3
    3
}


fn mux(flag: bool, byte1: u8, byte2: u8) -> u8 {
    if flag {byte1} else {byte2}
}


fn inc(x: i32) -> i32 {
    x + 1
}


fn main() {
    val_printer(3);
    two_val_printer(1, 0);
    println!("implicitly returned: {}", three());
    val_printer(inc(10));
    // expressions

    let y = {
        // this block is an expression which returns 2
        let x = 1;
        x + 1
    };

    println!("expression evaluated to: {}", y);

    println!("mux {{22,100}} asserted: {}", mux(true, 22, 100));
    println!("mux {{22,100}} not asserted: {}", mux(false, 22, 100));

}
