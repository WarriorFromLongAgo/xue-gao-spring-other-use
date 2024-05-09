#[cfg(test)]
mod struct_test_1 {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    #[test]
    fn struct_test_1() {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
        // = help: the trait `std::fmt::Display` is not implemented for `User`
        // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
        // = note: this error originates in the macro `$crate::format_args_nl` which comes from
        //      the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        // println!("user1 {user1}")

        user1.email = String::from("anotheremail@example.com");

        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };
        let user3 = User {
            email: String::from("another@example.com"),
            ..user2
        };
        // 单行
        println!("user3 {:?}", user3);
        // 多行
        println!("user3 {:#?}", user3);
    }
}