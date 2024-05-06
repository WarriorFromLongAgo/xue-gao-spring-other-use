use std::cmp::Ordering;
use std::io;

use rand::Rng;

mod my_mod;
mod my_mod_v1;

fn main() {
    test_int();
    test_string();
    test_io();
    test_rand();
    test_cmp();
}

fn test_int() {
    // mut 表示可变，变量默认是不可以变的
    let mut foo: i32 = 1;
    let bar = foo;
    // bar = 2
    println!("foo {}", foo);
    println!("bar {}", bar);

    let i32_32: i32 = 111;
    let i64_64: i64 = 111;
    let u32_32: u32 = 111;
    let u8_8: u8 = 111;
    let f32_32: f32 = 111.1;
    let f64_64: f64 = 111.1;

    let bool_flag = true;
}

fn test_string() {
    println!("sfdshjfsdhf");

    let one = "one";
    let one = "one";

    println!("one {}", one);
    println!("one {}", one);

    let char_char = 'o';
}

fn test_io() {
    println!("======= {} ======= ", "test_io");
    println!("======= {} ======= ", "test_io");
    println!("======= {} ======= ", "test_io");

    // let mut test_str = String::new();
    // io::stdin().read_line(&mut test_str).expect("无法读取");
    // io::stdin().read_line(&mut test_str).expect("无法读取");
    // println!("输入的数据是 {}", test_str);
}

fn test_rand() {
    println!("======= {} ======= ", "test_rand");
    println!("======= {} ======= ", "test_rand");
    println!("======= {} ======= ", "test_rand");

    let rand_str = rand::thread_rng().gen_range(1..101);
    println!("随机的数据是 {}", rand_str);
}

fn test_cmp() {
    let rand_int = rand::thread_rng().gen_range(1..101);
    let mut test_str = String::new();
    io::stdin().read_line(&mut test_str).expect("无法读取");

    let test_int: u32 = test_str.trim().parse().expect("请输入整数");
    match rand_int.cmp(&test_int) {
        Ordering::Less => { println!("小于") }
        Ordering::Equal => { println!("等于") }
        Ordering::Greater => { println!("大于") }
    }

    let test_int_v2: u32 = match test_str.trim().parse() {
        Ok(num) => { num }
        Err(err) => {
            match err {
                // 如果解析错误是 ParseIntError 类型
                // 打印错误信息
                // ParseIntError => println!("解析整数错误: {}", err),
                // 如果不是 ParseIntError 类型
                // 打印默认的错误信息
                _ => println!("发生了一个错误: {}", err),
            }
            // 在这里返回一个默认值，或者根据实际需求进行处理
            0 // 例如，如果解析失败，可以将 test_int_v2 设置为默认值 0
        }
    };

    println!("test_int_v2 {}", test_int_v2)
}

fn test_while() {
    loop {
        println!("djashdjahjdj");
        let mut test_str = String::new();
        io::stdin().read_line(&mut test_str).expect("无法读取");
        println!("test_str {}", test_str);

        break;
    }
}

fn test_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 11);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
}

fn test_arr() {
    let arr_1: [i32; 5] = [12, 26, 332, 446, 584];
}

fn function() {
    println!("called `function()`");
}

fn test_my_mod() {
    println!("======= {} ======= ", "test_my_mod");
    println!("======= {} ======= ", "test_my_mod");
    println!("======= {} ======= ", "test_my_mod");

    my_mod::function();
    // my_mod::private_function();

    function();

    my_mod::indirect_access();

    my_mod::nested::function();
    // my_mod::nested::private_function();

    // my_mod::inaccessible;
}