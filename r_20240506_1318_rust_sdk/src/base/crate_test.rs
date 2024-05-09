
#[cfg(test)]
pub mod crate_test_1 {
    // crate 是 Rust 在编译时最小的代码单位。如果你用 rustc 而不是 cargo 来编译一个文件（第一章我们这么做过），
    // 编译器还是会将那个文件认作一个 crate。crate 可以包含模块，模块可以定义在其他文件，然后和 crate 一起编译，我们会在接下来的章节中遇到。
    // crate 指的都是库，这与其他编程语言中 library 概念一致。

    // 包（package）是提供一系列功能的一个或者多个 crate。一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。
    // 包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 crate(binary crate)，但是必须至少包含一个 crate（无论是库的还是二进制的）。

    // backyard
    // ├── Cargo.lock
    // ├── Cargo.toml
    // └── src
    // ├── garden
    // │    └── vegetables.rs
    // ├── garden.rs
    // └── main.rs

    // crate
    // └── front_of_house
    //      ├── hosting
    //      │   ├── add_to_waitlist
    //      │   └── seat_at_table
    //      └── serving
    //          ├── take_order
    //          ├── serve_order
    //          └── take_payment


    // 引用模块项目的路径
    // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    // front_of_house::hosting::add_to_waitlist();

    // super 获取上一级

    // 使用 use 关键字将路径引入作用域
    // mod front_of_house {
    //     pub mod hosting {
    //         pub fn add_to_waitlist() {}
    //     }
    // }
    // use crate::front_of_house::hosting;
    // pub fn eat_at_restaurant() {
    //     hosting::add_to_waitlist();
    // }

    // use std::collections::HashMap;
    // use std::fmt::Result;
    // use std::io::Result as IoResult;




}