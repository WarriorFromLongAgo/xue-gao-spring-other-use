
#[cfg(test)]
mod option_test_1 {

    #[test]
    fn option_test_1() {
        let some_number = Option::Some(5);
        let some_char = Option::Some('e');

        let absent_number: Option<i32> = Option::None;

        let x: i8 = 5;
        let y: Option<i8> = Option::Some(5);

        let sum = x + y;


    }


}