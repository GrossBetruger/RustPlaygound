mod outermost {
    pub fn middle_function() {

    }

    fn middle_secret_function() {

    }

    mod inside {
        pub fn inner_function() {}

        fn inner_secret_function() {}
    }
}

pub mod super_nested {
    pub mod nest_a {
        pub mod nest_b {
            pub fn execute_me () {
                println!("touché")
            }

            pub fn dare_you_execute_me() {
                println!("I double dare you!")
            }
        }
    }
}

pub enum TrafficLights {
    Red,
    Yellow,
    Green
}

fn try_me() {
    outermost::middle_function(); // compiles even though outermost is private, because try_me is in outermost's root
//    outermost::middle_secret_function(); // won't compile - private
//    outermost::inside::inner_function(); // won't compile - inside private module
//    outermost::inside::inner_secret_function(); // won't compile - inside private module
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
