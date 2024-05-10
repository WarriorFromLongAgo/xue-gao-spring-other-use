#[cfg(test)]
mod life_time_test_2 {
    #[test]
    fn life_time_test_2() {
        let r;

        {
            let x = 5;
            // borrowed value does not live long enough
            // 变量 x 并没有 “存在的足够久”。
            r = &x;
        }

        // println!("r: {}", r);
    }

    // 生命周期注解语法
    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用



}