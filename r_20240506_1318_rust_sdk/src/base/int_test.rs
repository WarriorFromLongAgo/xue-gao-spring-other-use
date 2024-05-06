fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn internal() {
        let res = internal_adder(2, 2);
        println!("Hello, world!");
        assert_eq!(4, res);
    }
}
