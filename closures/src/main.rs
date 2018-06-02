use std::thread;
use std::time::Duration;


struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl <T> Cacher <T>
    where T: Fn(u32) -> u32
    {

    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(val) => val,
            None => {
                let val = (self.calculation)(arg);
                self.value = Some(val);
                val
            }
        }
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
