use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value_map: HashMap<u32, u32>
}

impl <T> Cacher <T>
    where T: Fn(u32) -> u32
    {

    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value_map: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value_map.get(&arg) {
            Some(val) => return *val,
            None => {}
        }
        *self.value_map.entry(arg)
            .or_insert({
                       let val = (self.calculation)(arg);
                       val
                       })
        }

    }

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_my_size(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|shoe| shoe.size == my_size)
        .collect()
}

fn simulate_expensive_calc(intensity: u32) -> u32 {
    println!("calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_factor: u32) {

    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("today, do {} pushups!",
                 expensive_closure.value(intensity));


        println!("next do {} situps!",
                 expensive_closure.value(intensity))
    }

    else {
        if random_factor == 3 {
            println!("take a break today, think about life")
        }

        else {
            println!("today, run for {} minutes!",
                     expensive_closure.value(intensity))
        }
    }
}


fn main() {

    generate_workout(
        10, 7
    );

    let pi = 3.141592;

    let equals_pi = |num| num == pi; // The closure is allowed to access 'pi' from its scope

    println!("{} is pi: {}", 3.141, equals_pi(3.141));
    println!("{} is pi: {}", 3.141592, equals_pi(3.141592));

//    fn ten_pi() -> i32 {pi * 10}; // won't compile - only closure can access parent scope

    let v = vec![7, 7, 7];

    let equals_v = move |vec| vec == v;

    assert!(equals_v(vec![7,7,7]));

//    println!("v: {:?}", v); // won't compile because 'equals_v' takes ownership of v


    let primes: Vec<i32> = vec![3, 5, 7];
    let evens = primes.iter().map(|prime| prime + 1);

    let evaluated_evens = evens.collect::<Vec<i32>>();

    println!("evened primes: {:?}", evaluated_evens);




}

mod tests {

    use super::*;

    #[test]
    fn cacher_different_args() {
        let mut cacher = Cacher::new(|num: u32| {
            println!("input: {}, takes time...", num);
            thread::sleep(Duration::from_secs(1));
            num + 0
        });

        let inpt1 = 1;
        let inpt2 = 2;
        let v1 = cacher.value(inpt1);
        let v2 = cacher.value(inpt2);
        let v3 = cacher.value(inpt2);
        let v4 = cacher.value(inpt2);
        let v5 = cacher.value(inpt2);
        let v6 = cacher.value(inpt2);
        let v7 = cacher.value(inpt2);
        let v8 = cacher.value(inpt2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        assert_eq!(v3, 2);
        assert_eq!(v4, 2);
        assert_eq!(v5, 2);
        assert_eq!(v6, 2);
        assert_eq!(v7, 2);
        assert_eq!(v8, 2);
    }

    #[test]
    fn test_laziness() {
        println!("test no evaluation");
        let mega_vector = 1..10u64.pow(18u32);
        println!("done no evaluation")

    }

    #[test]
    fn test_laziness2() {
        println!("test with evaluation");
        let mega_vector = 1..10u64.pow(8u32);
        // of optimization level is >= 2 in profile.test this will be ignored
        // by the compiler as trivial and won't take any runtime
        for i in mega_vector{

        }
        println!("{}",String::from("done evaluating... phew, that was hard!").to_uppercase());

    }

    #[test]
    fn test_iter_next() {
        let vec = vec![0, -1, -2];
        let mut v = vec.iter();

        assert_eq!(v.next(), Some(&0));
        assert_eq!(v.next(), Some(&-1));
        assert_eq!(v.next(), Some(&-2));
        assert_eq!(v.next(), None);
    }

    #[test]
    fn test_consuming_adaptors() {
        let v: Vec<u32> = vec![8, 8, 1];

        let i = v.iter();
        let total: u32 = i.sum();
        assert_eq!(total, 17);
//        println!("iterator {:?}", i); // won't compile value moved (by calling sum)
    }

    #[test]
    fn test_shoes_sieve() {
        let shoes = vec![Shoe {size: 32, style: String::from("sneaker")},
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") }];

        let fitting = shoes_my_size(shoes, 32);
        assert_eq!(fitting, vec![Shoe {size: 32, style: String::from("sneaker")}])
    }
    
}



