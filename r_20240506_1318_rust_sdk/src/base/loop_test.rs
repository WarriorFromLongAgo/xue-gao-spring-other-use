#[cfg(test)]
pub mod loop_test_1 {
    #[test]
    fn loop_test_1() {
        let mut one = 1;
        loop {
            one = one + 1;
            if one > 100 {
                println!("one {one}");
                break;
            }
        }
    }

    #[test]
    fn loop_test_2() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }

    #[test]
    fn loop_test_3() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }
}