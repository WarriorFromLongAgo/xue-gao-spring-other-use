#[cfg(test)]
mod impl_test_1 {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    #[test]
    fn impl_test_1() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("The area of the rectangle is {} square pixels.", rect1.area());
    }
}