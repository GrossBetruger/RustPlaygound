use std::mem::forget;
use std::{thread, time};

struct StupidPointer {
    data: String
}

impl Drop for StupidPointer {
    fn drop(&mut self) {
        println!("lazy cleanup");
    }
}

fn main() {
    let mut counter = 0;
    loop {

        counter += 1;

        let mem_worm = StupidPointer{data: String::from("6bytes6bytes6bytes6bytes6bytes6bytes6bytes")};
        forget(mem_worm);

        let milisecond = time::Duration::from_millis(1);
        if (counter % 1000 == 0) {
            thread::sleep(milisecond);
        }

    }
}
