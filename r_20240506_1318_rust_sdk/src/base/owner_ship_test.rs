#[cfg(test)]
pub mod owner_ship_test_1 {
    #[test]
    fn owner_ship_test_1() {
        {
            // s 在这里无效，它尚未声明
            // 从此处起，s 是有效的
            let s = "hello";

            // 使用 s
        }
        // 此作用域已结束，s 不再有效
    }

    #[test]
    fn owner_ship_test_1_str() {
        let mut s = String::from("hello");

        // push_str() 在字符串后追加字面值
        s.push_str(", world!");

        // 将打印 `hello, world!`
        println!("{}", s);
    }

    #[test]
    fn owner_ship_test_move() {
        let x = 5;
        let y = x;
        println!("x {x}");
        println!("y {y}");

        // move 发生的原因是 `s1` 的类型是 `String` ，而 `String` 没有实现 `Copy` 特性
        // move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        // let s1 = String::from("hello");
        // let s2 = s1;
        // println!("s1 {s1}");
        // println!("s2 {s2}");

        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 {s1}");
        println!("s2 {s2}");
    }

    #[test]
    fn owner_ship_test_move_func() {
        // s1 进入作用域
        let s1 = String::from("hello");
        println!("s1 {s1}");
        // s 的值移动到函数里 ...
        owner_ship_test_move_func_test_1(s1);
        // ... 所以到这里不再有效

        // x 进入作用域
        let x = 5;
        owner_ship_test_copy_func_test_1(x);
        // x 应该移动函数里，但 i32 是 Copy 的，
        // 所以在后面可继续使用 x
    }
    // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 没有特殊之处

    // s1 进入作用域
    fn owner_ship_test_move_func_test_1(s1: String) {
        println!("s1 {s1}");
    }
    // 这里，some_string 移出作用域并调用 `drop` 方法。
    // 占用的内存被释放

    // s1 进入作用域
    fn owner_ship_test_copy_func_test_1(s1: i32) {
        println!("{}", s1);
    }
    // 这里，s1 移出作用域。没有特殊之处

    // 返回值与作用域
    #[test]
    fn owner_ship_test_return_move() {
        // gives_ownership 将返回值
        let s1 = gives_ownership();
        // 转移给 s1

        // s2 进入作用域
        let s2 = String::from("hello");

        // s2 被移动到 takes_and_gives_back 中，
        let s3 = takes_and_gives_back(s2);
        // 它也将返回值移给 s3
    }
    // 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
    // 所以什么也不会发生。s1 离开作用域并被丢弃

    fn gives_ownership() -> String {
        // gives_ownership 会将返回值移动给
        // 调用它的函数

        // some_string 进入作用域。
        let some_string = String::from("yours");

        // 返回 some_string
        return some_string;
        // 并移出给调用的函数
    }

    // takes_and_gives_back 将传入字符串并返回该值
    // a_string 进入作用域
    fn takes_and_gives_back(a_string: String) -> String {
        //

        // 返回 a_string 并移出给调用的函数
        return a_string;
    }
}