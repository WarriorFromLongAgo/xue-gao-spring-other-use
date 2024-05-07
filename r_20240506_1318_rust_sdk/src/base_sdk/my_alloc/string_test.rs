#[cfg(test)]
mod string_test_1 {
    #[test]
    fn string_test_1() {
        let guess = String::new();

        println!("You guessed: {guess}");
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
}