
pub fn add_one(n: i32) -> i32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}
