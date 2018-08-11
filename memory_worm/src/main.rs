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
    loop {

        let mem_worm = StupidPointer{data: String::from("6bytes6bytes6bytes6bytes6bytes6bytes6bytes")};
        forget(mem_worm);

        let milisecond = time::Duration::from_millis(1);
//        thread::sleep(milisecond);

    }
}
