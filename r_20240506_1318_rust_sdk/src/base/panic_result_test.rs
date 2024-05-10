#[cfg(test)]
pub mod panic_result_test_1 {
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
}