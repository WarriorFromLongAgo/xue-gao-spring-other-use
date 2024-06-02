#[cfg(test)]
pub mod iflet_test_1 {
    #[test]
    fn iflet_test_1() {
        // let favorite_color: Option<&str> = Some("1111");
        let favorite_color: Option<&str> = None;

        let is_tuesday = false;
        // let is_tuesday = true;

        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    #[test]
    fn while_let_test_1() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
}