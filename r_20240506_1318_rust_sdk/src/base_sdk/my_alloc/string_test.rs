#[cfg(test)]
mod string_test_1 {
    #[test]
    fn string_test_1() {
        let guess = String::new();

        println!("You guessed: {guess}");

        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("You hello: {hello}");
        println!("You world: {world}");
    }

    #[test]
    fn string_format_test_1() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");

        println!("You s: {s}");

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("You s: {s}");
    }

    #[test]
    fn string_parse_test_1() {
        let guess = "dasdad";
        println!("You guess: {guess}");

        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                // 如果解析失败，打印错误信息
                println!("Error : {e}");
                // 你可能想根据实际需求以不同方式处理错误，例如返回错误值或 panic。
                // 在这里我们选择提前返回，结束函数的执行。
                return;
            }
        };
        println!("You guess_int: {guess_int}");
        // 如果解析成功，可以在这里继续执行后续的测试逻辑
    }

    #[test]
    fn string_slice_test_1() {
        let hello = "Здравствуйте";

        let s = &hello[0..4];
        println!("You s: {s}");
        // You s: Зд
    }

    #[test]
    fn string_for_test_1() {
        for c in "Зд".chars() {
            println!("{c}");
        }
        for b in "Зд".bytes() {
            println!("{b}");
        }
    }

    #[test]
    fn string_test_11() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}