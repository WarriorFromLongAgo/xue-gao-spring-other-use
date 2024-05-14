#[cfg(test)]
pub mod weak_test {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use std::thread;

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[test]
    fn weak_test() {
        // 强引用代表如何共享 Rc<T> 实例的所有权。弱引用并不属于所有权关系，当 Rc<T> 实例被清理时其计数没有影响。
        // 它们不会造成引用循环，因为任何涉及弱引用的循环会在其相关的值的强引用计数为 0 时被打断。

        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });

        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    }

    #[derive(Debug)]
    struct NodeV2 {
        value: i32,
        parent: RefCell<Weak<NodeV2>>,
        children: RefCell<Vec<Rc<NodeV2>>>,
    }

    #[test]
    fn weak_test_2() {
        let leaf = Rc::new(NodeV2 {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(NodeV2 {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
}