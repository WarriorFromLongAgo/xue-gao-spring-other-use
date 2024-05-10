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



    #[derive(Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
}