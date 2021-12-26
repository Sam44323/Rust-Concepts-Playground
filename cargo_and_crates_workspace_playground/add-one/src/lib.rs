pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(add_one(3), 4);
    }
}
