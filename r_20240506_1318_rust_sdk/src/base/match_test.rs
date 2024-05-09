#[cfg(test)]
pub mod match_test_1 {
    use std::cmp::Ordering;

    #[test]
    fn match_cmp() {
        let one = 1;
        let two = 2;

        match one.cmp(&two) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    #[test]
    fn match_enum() {
        let coin = Coin::Dime;

        let a = match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };
        println!("a {a}")
    }

    #[test]
    fn match_option() {
        let five = Some(5);
        let six = plus_one(five);
        println!("six {:?}", six);
        println!("six {:?}", six.unwrap());

        let none = plus_one(None);
        println!("none {:?}", none);
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}