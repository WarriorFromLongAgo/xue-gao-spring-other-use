#[cfg(test)]
pub mod mut_test_1 {
    #[test]
    fn mut_test_1() {
        let x = 5;
        println!("The value of x is: {x}");
        // x = 6;
        println!("The value of x is: {x}");
    }

    #[test]
    fn mut_test_2() {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }
}