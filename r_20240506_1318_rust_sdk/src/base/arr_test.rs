#[cfg(test)]
pub mod arr_test_1 {
    #[test]
    fn arr_test_1() {
        let a = [1, 2, 3, 4, 5];
        // println!("a {:#?}", a);
        println!("a {:#?}", a)
    }

    #[test]
    fn arr_test_2() {
        let s = "abcdefg";
        let bytes = s.as_bytes();

        for x in bytes.iter() {
            println!("1111111111 {x} ")
        }

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                println!("bbbbbb {i} {item}")
            }
            // if item == 'b' {
            //     println!("bbbbbb {i} {item}")
            // }
            println!("2222 {i} {item}")
        }
    }
}