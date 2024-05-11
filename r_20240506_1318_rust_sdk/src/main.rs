use std::env;

mod rand;
mod base;
mod base_sdk;

fn main() {
    println!("Hello, world!");

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let file_path = &args[2];
    // println!("Searching for {}", query);
    // println!("In file {}", file_path);
    println!("In file {:?}", args);


}


