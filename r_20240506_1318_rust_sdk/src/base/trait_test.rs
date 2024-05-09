// 除了 Debug trait，Rust 还为我们提供了很多可以通过 derive 属性来使用的 trait，它们可以为我们的自定义类型增加实用的行为。附录 C 中列出了这些 trait 和行为。
// 可派生的 trait: https://kaisery.github.io/trpl-zh-cn/appendix-03-derivable-traits.html

// 除了 derive 之外，还有很多属性；更多信息请参见 Rust Reference 的 Attributes 部分。
// 属性 https://doc.rust-lang.org/stable/reference/attributes.html

#[cfg(test)]
mod trait_test_1 {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    #[test]
    fn trait_test_1() {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        println!("rect1 is {:?}", rect1);
    }
}