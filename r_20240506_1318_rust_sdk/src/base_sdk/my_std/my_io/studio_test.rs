#[cfg(test)]
mod studio_test_1 {
    use std::io;

    // cargo test -- --show-output
    // cargo test -- --nocapture
    #[test]
    fn stdin_read_line_1() {
        let mut guess = String::new();
        println!("You guessed: {guess}");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
    }
}