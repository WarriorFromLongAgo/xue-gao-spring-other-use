#[cfg(test)]
pub mod rand_test {
    #[test]
    fn internal() {
        let res = internal_adder(2, 2);
        println!("Hello, world!");
        assert_eq!(4, res);
    }
}

