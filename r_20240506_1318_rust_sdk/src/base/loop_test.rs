#[cfg(test)]
pub mod loop_test_1 {
    #[test]
    fn loop_test_1() {
        let mut one = 1;
        loop {
            one = one + 1;
            if one > 100 {
                println!("one {one}");
                break;
            }
        }
    }
}