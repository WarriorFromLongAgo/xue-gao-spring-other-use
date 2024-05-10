#[cfg(test)]
pub mod generic_types_test_1 {
    #[test]
    fn generic_types_test_1() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // <T: std::cmp::PartialOrd>
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    struct Point2<T, U> {
        x: T,
        y: U,
    }

    #[test]
    fn generic_types_test_2() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        println!("p.x = {}", float.x());
        println!("The largest char is {:?}", integer);
        println!("The largest char is {:?}", float);

        // 字段 x 和 y 的类型必须相同，因为它们都有相同的泛型类型 T
        // let wont_work = Point { x: 5, y: 4.0 };
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        println!("The largest char is {:?}", both_integer);
        println!("The largest char is {:?}", both_float);
    }
}