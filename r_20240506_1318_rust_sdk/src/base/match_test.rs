#[cfg(test)]
pub mod match_test_1 {
    use std::cmp::Ordering;

    #[test]
    fn match_cmp() {
        let one = 1;
        let two = 2;

        match one.cmp(&two) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}