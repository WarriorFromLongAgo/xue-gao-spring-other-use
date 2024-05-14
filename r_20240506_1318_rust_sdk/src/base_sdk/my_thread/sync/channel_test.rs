#[cfg(test)]
mod channel_test_1 {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn channel_test_client_1() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
    }

    #[test]
    fn channel_test_server_1() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
}