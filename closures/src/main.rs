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
    )
}

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
