#[cfg(test)]
pub mod owner_ship_test_v2 {
    // 引用
    // 借用

    #[test]
    fn owner_ship_test_v2_1() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    // s 是 String 的引用
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
    // 所以什么也不会发生

    #[test]
    fn owner_ship_test_v2_2() {
        let mut s = String::from("hello");
        println!("The length of '{}' is {}.", s, s.len());

        change(&mut s);
        println!("The length of '{}' is {}.", s, s.len());
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    #[test]
    fn owner_ship_test_v2_3() {
        let mut s = String::from("hello");

        // let r1 = &s; // 没问题
        // let r2 = &s; // 没问题
        // let r3 = &mut s; // 大问题
        // 我们 也 不能在拥有不可变引用的同时拥有可变引用。

        // println!("{}, {}, and {}", r1, r2, r3);
    }

    #[test]
    fn owner_ship_test_v2_4() {
        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用
        // 我们 也 不能在拥有不可变引用的同时拥有可变引用。

        let r3 = &mut s; // 没问题
        println!("{}", r3);
    }
}