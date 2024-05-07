#[cfg(test)]
pub mod tup_test_1 {
    #[test]
    fn tup_test_1() {
        let tup = (500, 6.4, 1);

        let (_, y, _) = tup;

        println!("The value of y is: {y}");


        // let x: (i32, f64, u8) = (500, 6.4, 1);
        //
        // let five_hundred = x.0;
        //
        // let six_point_four = x.1;
        //
        // let one = x.2;
    }
}