#[cfg(test)]
pub mod generic_types_test_1 {
    fn generic<T>(t: T) {
        // --snip--
    }

    fn generic_sized<T: Sized>(t: T) {
        // --snip--
    }

    fn generic_unsized<T: ?Sized>(t: &T) {
        // --snip--
    }

    #[test]
    fn sized_1() {}
}