#[cfg(test)]
pub mod let_const_test_1 {
    #[test]
    fn let_test_1() {
        let one = 1;
        println!("one {one}");
    }

    #[test]
    fn let_test_2() {
        let x = 5;
        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");
    }

    #[test]
    fn let_test_3() {
        let spaces = "   ";
        let spaces = spaces.len();

        println!("The value of spaces is: {spaces}");
    }

    #[test]
    fn let_char_test_3() {
        let spaces: char = 'a';
        println!("The value of spaces is: {spaces}");
    }

    #[test]
    fn const_test_1() {
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

        println!("THREE_HOURS_IN_SECONDS {THREE_HOURS_IN_SECONDS}");
    }
}