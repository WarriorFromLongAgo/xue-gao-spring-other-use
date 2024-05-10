// 除了 Debug trait，Rust 还为我们提供了很多可以通过 derive 属性来使用的 trait，它们可以为我们的自定义类型增加实用的行为。附录 C 中列出了这些 trait 和行为。
// 可派生的 trait: https://kaisery.github.io/trpl-zh-cn/appendix-03-derivable-traits.html

// 除了 derive 之外，还有很多属性；更多信息请参见 Rust Reference 的 Attributes 部分。
// 属性 https://doc.rust-lang.org/stable/reference/attributes.html

#[cfg(test)]
mod trait_test_1 {
    use std::fmt::{Debug, Display};

    #[derive(Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub trait Summary {
        fn summarize(&self) -> String;

        // 默认实现
        fn summarize_default(&self) -> String {
            String::from("(Read more...)")
        }
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }


    #[test]
    fn trait_test_1() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
        println!("1 new tweet: {}", tweet.summarize_default());


        notify(&tweet);
        notify_v2(&tweet);

        // let rect1 = Rectangle {
        //     width: 30,
        //     height: 50,
        // };
        // notify(&rect1);
    }

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

    // trait 作为参数
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait Bound 语法
    pub fn notify_v2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify_v3<T: Summary>(item1: &T, item2: &T) {
        println!("Breaking news! {} {}", item2.summarize(), item1.summarize_default());
    }

    pub fn notify_v10(item: &(impl Summary + Display)) {}

    // 通过 + 指定多个 trait bound
    pub fn notify_v11<T: Summary + Display>(item: &T) {}

    // 分割线
    // 通过 where 简化 trait bound
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { 0 }
    // 通过 where 简化 trait bound
    fn some_function_v2<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
    {
        0
    }


    // 返回实现了 trait 的类型
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }

    // fn returns_summarizable_v2(switch: bool) -> impl Summary {
        // if switch {
            // NewsArticle {
            //     headline: String::from(
            //         "Penguins win the Stanley Cup Championship!",
            //     ),
            //     location: String::from("Pittsburgh, PA, USA"),
            //     author: String::from("Iceburgh"),
            //     content: String::from(
            //         "The Pittsburgh Penguins once again are the best \
            //      hockey team in the NHL.",
            //     ),
            // }
        // } else {
            // Tweet {
            //     username: String::from("horse_ebooks"),
            //     content: String::from(
            //         "of course, as you probably already know, people",
            //     ),
            //     reply: false,
            //     retweet: false,
            // }
        // }
    // }

    // 使用 trait bound 有条件地实现方法



}