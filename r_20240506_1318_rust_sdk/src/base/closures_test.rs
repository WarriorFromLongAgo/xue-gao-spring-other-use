#[cfg(test)]
pub mod closures_test_1 {
    // https://channaly.medium.com/the-differences-between-function-pointers-and-closures-how-to-rust-b80abd503856

    use std::thread;

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }


    #[test]
    fn closures_test_1() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }

    #[test]
    fn closures_test_2() {
        // let expensive_closure = |num: u32| -> u32 {
        //     println!("calculating slowly...");
        //     my_thread::sleep(Duration::from_secs(2));
        //     num
        // };

        let example_closure = |x| x;
        // 编译器会为闭包定义中的每个参数和返回值推断一个具体类型。
        let s = example_closure(String::from("hello"));
        // let n = example_closure(5);
        // 第一次使用 String 值调用 example_closure 时，编译器推断这个闭包中 x 的类型以及返回值的类型是 String。
        // 接着这些类型被锁定进闭包 example_closure 中，如果尝试对同一闭包使用不同类型则就会得到类型错误。

        let fn_pointer: fn(i32) -> i32 = add_one;
        println!("Function pointer output: {}", fn_pointer(2));
    }

    #[test]
    fn closures_test_3() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    #[test]
    fn closures_test_4() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);

        borrows_mutably();
        borrows_mutably();
        println!("After calling closure: {:?}", list);

        let list_of_strings_1: Vec<String> =
            list.iter().map(|i| i.to_string()).collect();
        let list_of_strings_2: Vec<String> =
            list.iter().map(|i| {
                println!(" i {i}");
                i.to_string()
            }).collect();
        println!("After calling closure:  list_of_strings  {:?}", list_of_strings_1);
        println!("After calling closure:  list_of_strings  {:?}", list_of_strings_2);
    }

    // 即使闭包体不严格需要所有权，如果希望强制闭包获取它用到的环境中值的所有权，可以在参数列表前使用 move 关键字。
    #[test]
    fn closures_test_5() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // 在将闭包传递到一个新的线程时这个技巧很有用，它可以移动数据所有权给新线程。
        // 我们生成了新的线程，给这个线程一个闭包作为参数运行，闭包体打印出列表。
        // 这个例子中，尽管闭包体依然只需要不可变引用，我们还是在闭包定义前写上 move 关键字来指明 list 应当被移动到闭包中。
        // 新线程可能在主线程剩余部分执行完前执行完，或者也可能主线程先执行完。
        // 如果主线程维护了 list 的所有权但却在新线程之前结束并且丢弃了 list，则在线程中的不可变引用将失效。
        // 因此，编译器要求 list 被移动到在新线程中运行的闭包中，这样引用就是有效的。
        // 试着去掉 move 关键字或在闭包被定义后在主线程中使用 list 看看你会得到什么编译器报错！

        let s = thread::spawn(move || {
            // list.push(7);
            println!("From my_thread: {:?}", list);
        })
            .join()
            .unwrap();

        // error
        // value borrowed here after move
        // println!("From my_thread: {:?}", list);
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // 将被捕获的值移出闭包和 Fn trait
    #[test]
    fn closures_test_6() {
        // 一旦闭包捕获了定义它的环境中一个值的引用或者所有权（也就影响了什么会被移 进 闭包，如有)，
        // 闭包体中的代码定义了稍后在闭包计算时对引用或值如何操作（也就影响了什么会被移 出 闭包，如有）。
        // 闭包体可以做以下任何事：将一个捕获的值移出闭包，修改捕获的值，既不移动也不修改值，或者一开始就不从环境中捕获值。

        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        list.sort_by_key(|r| r.width);
        println!("{:?}", list);
    }

    #[test]
    fn closures_test_7() {
        // 一旦闭包捕获了定义它的环境中一个值的引用或者所有权（也就影响了什么会被移 进 闭包，如有)，
        // 闭包体中的代码定义了稍后在闭包计算时对引用或值如何操作（也就影响了什么会被移 出 闭包，如有）。
        // 闭包体可以做以下任何事：将一个捕获的值移出闭包，修改捕获的值，既不移动也不修改值，或者一开始就不从环境中捕获值。

        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{:?}", list);
    }

    fn add_one(x: i32) -> i32 {
        println!("x {x}");
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        println!("arg {arg}");
        f(arg) + f(arg)
    }

    #[test]
    fn closures_test_8() {
        let answer = do_twice(add_one, 5);

        println!("The answer is: {}", answer);
    }

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    #[test]
    fn closures_test_returns_closures_1() {
        let aaaa = returns_closure();
        // 调用闭包
        let result = aaaa(5);
        // 验证结果是否正确
        assert_eq!(result, 6);
    }
}
