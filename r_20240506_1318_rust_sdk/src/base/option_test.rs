#[cfg(test)]
mod option_test_1 {
    #[test]
    fn option_test_1() {
        let some_number = Option::Some(5);
        let some_char = Option::Some('e');

        let absent_number: Option<i32> = Option::None;

        let x: i8 = 5;
        let y: Option<i8> = Option::Some(5);
        // let sum = x + y;
        // no implementation for `i8 + Option<i8>`

        let sum = x + y.unwrap();
    }

    fn most_stocked() -> String {
        String::from("1")
    }

    fn giveaway(user_preference: Option<String>) -> String {
        user_preference.unwrap_or_else(|| most_stocked())
    }

    fn giveaway_v2(user_preference: Option<String>) -> String {
        match user_preference {
            Some(color) => color,
            None => most_stocked(),
        }
    }

    #[test]
    fn option_test_2() {
        giveaway(Option::Some(String::from("11")));
    }
}