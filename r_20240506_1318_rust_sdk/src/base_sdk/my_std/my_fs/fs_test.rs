#[cfg(test)]
mod fs_test_1 {
    use std::fs;

    #[test]
    fn fs_test_1() {
        // --snip--
        println!("In file {}", "file_path");

        let contents = fs::read_to_string("./Cargo.toml")
            .expect("Should have been able to read the file");

        println!("With text:\n{contents}");
    }
}