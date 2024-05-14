#[cfg(test)]
mod thread_test_1 {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn thread_test_1() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned my_thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..5 {
            println!("hi number {} from the main my_thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
}