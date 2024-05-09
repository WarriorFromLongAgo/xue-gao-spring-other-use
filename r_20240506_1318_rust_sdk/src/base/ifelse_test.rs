#[cfg(test)]
pub mod ifelse_test_1 {
    #[test]
    fn ifelse_test_1() {
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

        if let number = 2 {
            println!("condition was true");
        }
        let v: Vec<i32> = Vec::new();
    }

    #[test]
    fn ifelse_test_2() {
        let condition = true;

        // let number = if condition { 5 } else { "six" };
        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }
}