pub mod shapes {
    pub struct Rectangle {
        pub width: (u32),
        pub height: (u32)
    }


    impl Rectangle {
        pub fn can_hold(&self, another_rect: &Rectangle) -> bool {
            self.width >= another_rect.width && self.height >= another_rect.height
        }
    }
}

#[cfg(test)]
mod tests {
    use shapes::Rectangle;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_can_hold() {
        let big_rect = Rectangle {width: 8, height: 6};
        let small_rect = Rectangle {width: 2, height: 6};
        assert!(big_rect.can_hold(&small_rect));
        assert!(!small_rect.can_hold(&big_rect));
    }
}
