const STARTUP_SUCCESS_RATE: f64 = 0.10;

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


fn binom(p: f64, n: f64, k: f64) -> f64{
    n_choose_k(n, k) * p.powf(k) * (1. - p).powf(n - k)
}

/// binom_k_greater_equals.
///
/// Description.
///
/// calculates the probability of 'k  or more' wanted events.
///
/// example: the probability of success in 4 or more out of 5 startups when success probability is
/// 0.1 => binom_k_larger_equals(0.1, 5., 4.)
fn binom_k_greater_equals(p: f64, n: f64, k: f64) -> f64 {
    let chance: f64 = (k as u64..n as u64+1_u64)
        .collect::<Vec<u64>>()
        .iter()
        .map(|_k| binom(p, n, *_k as f64))
        .sum();

    chance
}

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

    println!("binom of 3 girls and one boy in family of 5: {}", binom(0.5, 5., 3.));
    println!("80% success rate in 5 startups: {}", binom(STARTUP_SUCCESS_RATE, 5., 4.));
    println!("at least 80% success rate in 5 startups: {}", binom_k_greater_equals(STARTUP_SUCCESS_RATE, 5., 4.));
    println!("700 or more in psychometric :{}", binom_k_greater_equals(0.25, 90., 75.));

}
