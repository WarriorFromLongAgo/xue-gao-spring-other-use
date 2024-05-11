#[cfg(test)]
pub mod panic_result_test_1 {
    use std::{env, fs, process};
    use std::error::Error;
    use std::fs::File;
    use std::io::ErrorKind;

    #[test]
    fn panic_test_1() {
        panic!("crash and burn");
    }

    #[test]
    fn result_test_1() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file1 = match greeting_file_result {
            Ok(file) => {
                println!("asdadaddaad");
                file
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };

        // let greeting_file2 = File::open("hello.txt").unwrap_or_else(|error| {
        //     if error.kind() == ErrorKind::NotFound {
        //         File::create("hello.txt").unwrap_or_else(|error| {
        //             panic!("Problem creating the file: {:?}", error);
        //         })
        //     } else {
        //         panic!("Problem opening the file: {:?}", error);
        //     }
        // });
    }

    #[test]
    fn result_test_2() {
        let greeting_file = File::open("hello.txt")
            .expect("hello.txt should be included in this project");
    }

    struct Config {
        query: String,
        file_path: String,
    }

    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config { query, file_path })
        }
    }

    #[test]
    fn result_test_3() {
        // 从 new 中返回 Result 而不是调用 panic!
        let args: Vec<String> = env::args().collect();

        // let config_result = Config::build(&args);
        // // 使用unwrap_or_else()方法处理Result
        // let config = match config_result {
        //     Ok(config) => config, // 如果Result是Ok，则返回内部的值
        //     Err(err) => {
        //         // 如果Result是Err，则执行闭包中的逻辑
        //         println!("Error building config: {}", err);
        //         // 返回一个默认的Config，这里只是示例，你可以根据实际情况返回其他值
        //         Default::default()
        //     }
        // };

        // let config_result_2 = Config::build(&args);
        // let config_2 = config_result_2.unwrap_or_else(|err| {
        //     // 如果Result是Err，则执行闭包中的逻辑
        //     println!("Error building config: {}", err);
        //     // 返回一个默认的Config，这里只是示例，你可以根据实际情况返回其他值
        //     Default::default()
        // });
    }

    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        println!("With text:\n{contents}");

        Ok(())
    }

    #[test]
    fn result_test_4() {
        let config = Config {
            query: String::from("horse_ebooks"),
            file_path: String::from("horse_ebooks"),
        };

        println!("Searching for {}", config.query);
        println!("In file {}", config.file_path);

        if let Err(e) = run(config) {
            println!("Application error: {e}");
            process::exit(1);
        }
    }
}