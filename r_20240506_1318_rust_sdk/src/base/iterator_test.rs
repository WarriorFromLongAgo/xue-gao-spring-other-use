#[cfg(test)]
pub mod iterator_test {
    #[test]
    fn iterator_test() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }

        let total: i32 = v1.iter().sum();
        assert_eq!(total, 6);


        let v1_1: Vec<_> = v1.iter().map(|x| x + 1).collect();
        println!("Got: {:?}", v1_1);
    }
}