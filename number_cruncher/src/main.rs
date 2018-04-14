extern crate num;

use num::{BigInt, BigUint, Zero, One, FromPrimitive};

fn factorial(n: usize) -> BigInt {
    let mut f: BigInt = One::one();
    for i in 1..(n+1) {
        let bu: BigInt = FromPrimitive::from_usize(i).unwrap();
        f = f * bu;
    }
    f
}

fn n_choose_k(n: usize, k: usize) -> BigInt {
    factorial(n) / (factorial(k) * factorial(n - k))
}


fn birthday_probability(days: usize, n: usize) -> BigInt{
//    can't mix between BigInt and float types, won't work... :(
    let mut one: BigInt = One::one();
    let day_to_n = num::pow(days, n);
    println!("day to n: {}", day_to_n);
    let nom = factorial(n) * n_choose_k(days, n);
    println!("nom: {}", nom);
    one - (factorial(n) * n_choose_k(days, n) / num::pow(days, (n)))
}

fn main(){
    println!("fac 10: {}", factorial(4));
    println!("n choose k: {}", n_choose_k(255, 19));
    println!("birthday: {}", birthday_probability(365, 5));
}