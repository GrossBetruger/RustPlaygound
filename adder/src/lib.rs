pub mod misc {
    pub struct Rectangle {
        pub width: (u32),
        pub height: (u32)
    }

    pub struct User {
        age: (u8)
    }

    impl Rectangle {
        pub fn can_hold(&self, another_rect: &Rectangle) -> bool {
            self.width >= another_rect.width && self.height >= another_rect.height
        }
    }

    impl User {
        pub fn new(age: u8) -> User {
            if (age < 18) {
                panic!("user can't be underage")
            }

            User {
                age: age
            }
        }
    }

    pub fn increment(n: i32) -> i32 {
        n + 1
    }

    pub fn greet(name: &str) -> String {
        let mut prefix = String::from("hello ");
        prefix.push_str(name);
        prefix
    }

    pub fn print_arg_ret_0(arg: &str) -> u8 {
        println!("{}", arg);
        0
    }

    pub fn enumerate_ordinals(n: u64) {
        for i in 0..n {
            print!("{}", i);
        }
    }


}

#[cfg(test)]
mod tests {
    use misc::*;

    #[test]
    fn large_holds_small() {
        let big_rect = Rectangle {width: 8, height: 6};
        let small_rect = Rectangle {width: 2, height: 6};
        assert!(big_rect.can_hold(&small_rect));
    }

    #[test]
    fn small_cant_hold_large() {
        let big_rect = Rectangle {width: 8, height: 6};
        let small_rect = Rectangle {width: 2, height: 6};
        assert!(!small_rect.can_hold(&big_rect));
    }

    #[test]
    fn test_incrementer() {
        assert_eq!(42, increment(41));
    }

    #[test]
    fn test_incrementer_neg() {
        assert_eq!(-3, increment(-4));
    }

    #[test]
    fn test_change() {
        for i in -10..10 {
            assert_ne!(i, increment(i));
        }
    }

    #[test]
    fn test_greeting() {
        let name = "Bob";
        let greeting = greet(name);
        assert!(greeting.contains(name),
                format!("expected greeting to contain name: '{}'", name));
    }

    #[test]
    #[should_panic]
    fn test_underage_panic() {
        let user = User::new(10);
    }

    #[test]
    #[should_panic(expected = "user can't be underage")]
    fn test_underage_specific_panic() {
        let user = User::new(10);
    }

    #[test]
    fn test_valid_age() {
        let user = User::new(18);
    }

    #[test]
    fn test_print_ret() {
        assert_eq!(0, print_arg_ret_0("solid argument"));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        enumerate_ordinals(2_u64.pow(26));
    }
}
