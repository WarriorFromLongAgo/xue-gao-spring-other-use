#[cfg(test)]
mod trait_test_2 {
    use std::fmt;
    use std::fmt::Display;

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    #[test]
    fn trait_test_2_1() {
        // 完全限定语法定义为：<Type as Trait>::function(receiver_if_method, next_arg, ...);
        println!("A baby dog is called a {}", Dog::baby_name());
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    // 实现 OutlinePrint trait，它要求来自 Display 的功能
    //
    // 因为指定了 OutlinePrint 需要 Display trait，则可以在 outline_print 中使用 to_string，其会为任何实现 Display 的类型自动实现。
    // 如果不在 trait 名后增加 : Display 并尝试在 outline_print 中使用 to_string，
    // 则会得到一个错误说在当前作用域中没有找到用于 &Self 类型的方法 to_string。
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            println!(" 使用 outline_print 方法 ");
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            println!(" 使用 fmt 方法 ");
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}
    #[test]
    fn trait_test_2_2() {
        let point = Point {
            x: 11,
            y: 30,
        };
        println!(" 使用 println! 宏 ");
        println!("point = {}", point);
        println!(" ================================ ");
        println!(" ================================ ");
        println!(" ================================ ");

        println!(" 使用 format! 宏 ");
        // 使用 format! 宏
        let formatted_point = format!("{}", point);
        assert_eq!(formatted_point, "(11, 30)");
        println!(" ================================ ");
        println!(" ================================ ");
        println!(" ================================ ");

        println!(" 使用 to_string 方法 ");
        // 使用 to_string 方法
        let string_point = point.to_string();
        assert_eq!(string_point, "(11, 30)");
        println!(" ================================ ");
        println!(" ================================ ");
        println!(" ================================ ");

        println!(" 可选地，你还可以测试 outline_print 方法 ");
        // 可选地，你还可以测试 outline_print 方法
        point.outline_print();
        println!(" ================================ ");
        println!(" ================================ ");
        println!(" ================================ ");
    }

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    #[test]
    fn trait_test_2_3() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}