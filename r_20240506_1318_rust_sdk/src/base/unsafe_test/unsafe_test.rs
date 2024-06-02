#[cfg(test)]
pub mod unsafe_test_1 {
    use std::slice;

    #[test]
    fn unsafe_test_1() {
        // 解引用裸指针

        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        // 记得我们说过可以在安全代码中创建裸指针，不过不能 解引用 裸指针和读取其指向的数据。现在我们要做的就是对裸指针使用解引用运算符 *，这需要一个 unsafe 块，
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    unsafe fn dangerous() {
        println!("r2 is: {}", 1111);
    }

    #[test]
    fn unsafe_test_2() {
        // 调用不安全函数或方法
        unsafe {
            dangerous();
        }
    }

    #[test]
    fn unsafe_test_3() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[test]
    fn extern_test_1() {
        // Absolute value of -3 according to C: 3
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    #[test]
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        // Just called a Rust function from C!
        println!("Just called a Rust function from C!");
    }


    // 访问或修改可变静态变量
    static HELLO_WORLD: &str = "Hello, world!";
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    #[test]
    fn unsafe_test_4() {
        println!("name is: {}", HELLO_WORLD);

        // println!("COUNTER: {}", COUNTER);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }

        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }

    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    #[test]
    fn unsafe_test_5() {



    }
}