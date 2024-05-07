#[cfg(test)]
pub mod rand_test_1 {
    use rand::Rng;

    #[test]
    fn thread_rng_gen_range() {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Hello, world! {secret_number}");
    }
}

