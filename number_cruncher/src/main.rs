extern crate num;

use num::{BigInt, BigUint, Zero, One, FromPrimitive};

fn factorial(n: usize) -> BigUint {
    let mut f: BigUint = One::one();
    for i in 1..(n+1) {
        let bu: BigUint = FromPrimitive::from_usize(i).unwrap();
        f = f * bu;
    }
    f
}

fn n_choose_k(n: usize, k: usize) -> BigUint {
    factorial(n) / (factorial(k) * factorial(n - k))
}


fn birthday_probability(days: usize, n: usize) -> BigUint{
//    can't mix between BigInt and float types, won't work... :(
    let mut one: BigUint = One::one();
    one - ((factorial(n) * n_choose_k(days, n))) / num::pow(days, n)
}

fn main(){
    println!("fac 10: {}", factorial(100));
    println!("n choose k: {}", n_choose_k(255, 19));
    println!("birthday: {}", n_choose_k(255, 19));
}