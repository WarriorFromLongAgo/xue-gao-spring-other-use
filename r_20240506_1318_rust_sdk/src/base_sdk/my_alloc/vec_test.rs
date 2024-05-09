
#[cfg(test)]
mod vec_test_1 {
    #[test]
    fn vec_test_1() {
        let v: Vec<i32> = Vec::new();

        let v = vec![1, 2, 3];

        let mut v2 = Vec::new();
        v2.push(5);
        v2.push(6);
        v2.push(7);
        v2.push(8);

        let third: &i32 = &v2[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v2.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }

        // let does_not_exist = &v2[100];

        let mut v3 = vec![1, 2, 3, 4, 5];
        let first = &v3[0];
        // 如果把打印放到下面，就会报错
        // 对于可变引用和不可变引用的借用规则是非常严格的，
        println!("The first element is: {}", first);
        v3.push(6);
        // println!("The first element is: {}", first);

    }
}