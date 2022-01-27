pub fn add_nine(x: i32) -> i32 {
    x + 9
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(12, add_nine(3));
    }
}
