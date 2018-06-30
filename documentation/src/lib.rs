

/// Adds one to a give i32 num
///
/// # Examples
/// ```
/// use documentation::*;
///
/// let n = 3;
/// assert_eq!(add_one(3), 4);
/// ```


pub fn add_one(n: i32) -> i32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let n = 3;
        assert_eq!(add_one(3), 4);
    }
}
