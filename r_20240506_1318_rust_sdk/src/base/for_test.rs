#[cfg(test)]
pub mod for_test_1 {
    #[test]
    fn for_test_1() {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("the value is: {}", a[index]);

            index += 1;
        }
    }

    #[test]
    fn for_test_2() {
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }

        for number in (1..4).rev() {
            println!("2222222222   {number}!");
        }
    }
}