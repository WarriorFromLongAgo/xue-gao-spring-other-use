#[cfg(test)]
pub mod while_test_1 {
    #[test]
    fn while_test_1() {
        let mut number = 3;

        while number != 0 {
            println!("{number}!");

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }
}