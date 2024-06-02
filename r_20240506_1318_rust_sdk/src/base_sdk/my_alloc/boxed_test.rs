#[cfg(test)]
pub mod boxed_test {
    use std::rc::Rc;
    use List::{Cons, Nil};
    use crate::base_sdk::my_alloc::boxed_test::boxed_test::ListV2::{ConsV2, NilV2};

    #[test]
    fn boxed_test() {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    fn main() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    #[test]
    fn boxed_test_2() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list = {:?}", list);
    }

    #[derive(Debug)]
    enum ListV2 {
        ConsV2(i32, Rc<ListV2>),
        NilV2,
    }

    #[test]
    fn boxed_test_4() {
        // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        // let b = Cons(3, Box::new(a));
        // let c = Cons(4, Box::new(a));

        let a = Rc::new(ConsV2(5, Rc::new(ConsV2(10, Rc::new(NilV2)))));
        let b = ConsV2(3, Rc::clone(&a));
        let c = ConsV2(4, Rc::clone(&a));
    }


    #[test]
    fn boxed_test_3() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);

        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    pub trait Draw {
        fn draw(&self);
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // code to actually draw a button
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
        }
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,
    // }
    //
    // impl<T> Screen<T>
    //     where
    //         T: Draw,
    // {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }

    #[test]
    fn boxed_test_5() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();
    }
}