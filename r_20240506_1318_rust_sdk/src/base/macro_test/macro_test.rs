#[cfg(test)]
pub mod macro_test {
    // https://zjp-cn.github.io/tlborm/
    // 相关一些概念小宏书介绍得比较详细，可以看看这个 https://zjp-cn.github.io/tlborm/

    // 宏（Macro）指的是 Rust 中一系列的功能：

    // 使用 macro_rules! 的 声明（Declarative）宏，
    // 和
    // 三种 过程（Procedural）宏：
    // 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
    // 类属性（Attribute-like）宏定义可用于任意项的自定义属性
    // 类函数宏看起来像函数不过作用于作为参数传递的 token


    // 一个 vec! 宏定义的简化版本
    // let v: Vec<u32> = vec![1, 2, 3];
    #[macro_export]
    macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    }

    #[test]
    fn macro_test_1() {}

    // 过程宏（procedural macros
    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    // }

    // 编写自定义 derive 宏
    pub trait HelloMacro {
        fn hello_macro();
    }

    // #[derive(HelloMacro)]
    // struct Pancakes;
    //
    #[test]
    fn macro_test_2() {
        //     // Pancakes.he
    }
}