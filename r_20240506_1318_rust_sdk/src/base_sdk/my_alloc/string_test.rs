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

    #[test]
    fn string_test_12() {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }

    #[test]
    fn string_test_13() {
        // result = longest(string1.as_str(), string2.as_str());
        // |                                                ^^^^^^^ borrowed value does not live long enough

        // let string1 = String::from("long string is long");
        // let result;
        // {
        //     let string2 = String::from("xyz");
        //     result = longest(string1.as_str(), string2.as_str());
        // }
        // println!("The longest string is {}", result);
    }

    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // 深入理解生命周期
    // 深入理解生命周期
    // 深入理解生命周期

    // 如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期。如下代码将能够编译：
    fn longest_v9<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
    // 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值。
    // 然而它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。尝试考虑这个并不能编译的 longest 函数实现：
    // fn longest_v10<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     // returns a value referencing data owned by the current function
    //     result.as_str()
    // }

    // 传递所有权（Ownership）: 将拥有数据的所有权转移给调用函数。这样做会使得调用函数拥有数据的所有权，因此返回的引用将仍然有效。
    fn longest_v10(x: &str, y: &str) -> String {
        let result = String::from("really long string");
        // returns a value referencing data owned by the current function
        result
    }

    // 使用静态生命周期 'static': 如果您确定返回的数据在函数执行结束后仍然有效，并且其生命周期长于函数的生命周期，
    // 您可以将返回的引用标记为 'static' 生命周期。这会告诉编译器返回的引用将在整个程序的生命周期内有效。示例代码如下：
    fn longest_v11(x: &str, y: &str) -> &'static str {
        let result = "really long string";
        // 返回静态字符串
        result
    }

    #[test]
    fn string_test_14() {
        let result = "really long string".to_string();
        let result2 = "11111".to_string();
        // let test = result + &result2;
        // println!("test {test}");
        // println!("test {result}");
        // println!("test {result2}");
    }
}