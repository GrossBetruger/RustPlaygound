extern crate rand;
extern crate rayon;

use rand::Rng;
use rayon::prelude::*;


fn avg_parallel(list: &[f64]) -> f64 {
    for i in 1..500 {
        list.par_iter().sum::<f64>() / list.len() as f64;
    }
    list.par_iter().sum::<f64>() / list.len() as f64
}

fn avg(list: &[f64]) -> f64 {
   list.iter().sum::<f64>() / list.len() as f64
}

fn main() {
    const max_range: usize = 1_000_000;

    let mut l: [f64; max_range] = [0.; max_range];

    println!("generating random values...");

    for i in 0..max_range {
        l[i] = rand::thread_rng().gen_range(0., max_range as f64);
    }
    println!("calculating average serially...");
    println!("average: {}", avg(&l));


    println!("generating random values...");

    for i in 0..max_range {
        l[i] = rand::thread_rng().gen_range(0., max_range as f64);
    }

    println!("calculating average in parallel...");
    println!("average: {}", avg_parallel(&l));

}
