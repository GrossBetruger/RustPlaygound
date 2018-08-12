

fn factorial(n: f64) -> f64 {
    let mut result = 1.;
    for i in 1..n as u64 + 1 {
        result *= i as f64;
    }
    result
}


fn n_choose_k(n: f64, k: f64) -> f64 {
    factorial(n) / (factorial(k) * factorial(n - k))
}


//fn binom(p: f64, n: f64, k: f64) {
//    n_choose_k(n, k) * p.pow(k) * (1. - p).pow(n - k)
//}

fn main() {
    println!("fac 3: {}", factorial(3.));
    println!("fac 4: {}", factorial(4.));
    println!("fac 1: {}", factorial(1.));
    println!("fac 0: {}", factorial(0.));
    println!("fac 20: {}", factorial(20.));
    println!("12 choose 5: {}", n_choose_k(12., 5.));
    println!("19 choose 5: {}", n_choose_k(19., 5.));
    println!("19 choose 0: {}", n_choose_k(19., 0.));
    println!("19 choose 0: {}", n_choose_k(19., 0.));
}
