#[cfg(test)]
pub mod refcall_test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::base_sdk::my_alloc::refcall_test::refcall_test::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }


    // 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者

    #[test]
    fn refcall_test() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);

        // 这里创建了一个 Rc<RefCell<i32>> 实例并储存在变量 value 中以便之后直接访问。
        // 接着在 a 中用包含 value 的 Cons 成员创建了一个 List。
        // 需要克隆 value 以便 a 和 value 都能拥有其内部值 5 的所有权，而不是将所有权从 value 移动到 a 或者让 a 借用 value。
        //
        // 我们将列表 a 封装进了 Rc<T> 这样当创建列表 b 和 c 时，它们都可以引用 a，正如示例 15-18 一样。
        //
        // 一旦创建了列表 a、b 和 c，我们将 value 的值加 10。为此对 value 调用了 borrow_mut，
        // 这里使用了第五章讨论的自动解引用功能（“-> 运算符到哪去了？” 部分）来解引用 Rc<T> 以获取其内部的 RefCell<T>值。
        // borrow_mut 方法返回 RefMut<T> 智能指针，可以对其使用解引用运算符并修改其内部值。

    }
}