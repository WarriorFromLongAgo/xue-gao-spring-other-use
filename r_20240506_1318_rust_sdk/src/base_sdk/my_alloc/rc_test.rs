#[cfg(test)]
pub mod rc_test {
    use std::rc::Rc;

    use crate::base_sdk::my_alloc::rc_test::rc_test::ListV2::{ConsV2, NilV2};

    #[derive(Debug)]
    enum ListV2 {
        ConsV2(i32, Rc<ListV2>),
        NilV2,
    }

    #[test]
    fn rc_test_1() {
        let a = Rc::new(ListV2::ConsV2(5, Rc::new(ListV2::ConsV2(10, Rc::new(ListV2::NilV2)))));
        let b = ListV2::ConsV2(3, Rc::clone(&a));
        let c = ListV2::ConsV2(4, Rc::clone(&a));
    }

    // 打印引用计数
    #[test]
    fn rc_test_2() {
        let a = Rc::new(ConsV2(5, Rc::new(ConsV2(10, Rc::new(NilV2)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = ConsV2(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = ConsV2(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}