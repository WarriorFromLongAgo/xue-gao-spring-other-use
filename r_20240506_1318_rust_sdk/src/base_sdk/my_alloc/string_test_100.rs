#[cfg(test)]
mod string_test_100 {
    #[test]
    fn string_test_1() {
        let mut s = String::from("hello");
        let r1 = s.as_str();
        println!("r1  {}", r1);
        let r2 = &s;
        println!("r2  {}", r2);
        let r4 = first_word(r1);
        println!("{} and {}", r1, r2);

        let r3 = &mut s;
        println!("{} and {}", r3, r4);
    }

    // fn first_word(_s: &str) -> & str {
    //     "1"
    // }

    fn first_word(_s: &str) -> &'static str {
        // "1".to_string()
        // "1".parse().unwrap()
        "1"
    }


    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    #[test]
    fn string_test_2() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

    }
}