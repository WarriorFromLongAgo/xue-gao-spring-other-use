#[cfg(test)]
mod macros_test_1 {
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
    #[test]
    fn macros_test_1() {

        assert_eq!(4, add_two(2));
        // assert_eq!(5, add_two(2));
    }
}