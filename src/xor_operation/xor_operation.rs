pub fn xor_operation(n: i32, start: i32) -> i32 {
    (0..n).fold(0, |acc, i| acc ^ (start + 2 * i))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xor_operation() {
        assert_eq!(xor_operation(1, 7), 7);
        assert_eq!(xor_operation(5, 0), 8);
        assert_eq!(xor_operation(4, 3), 8);
        assert_eq!(xor_operation(10, 5), 2);
    }
}
