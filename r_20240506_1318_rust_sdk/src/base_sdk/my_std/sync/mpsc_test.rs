#[cfg(test)]
mod mpsc_test_1 {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn channel_1() {
        // mpsc::channel 函数返回一个元组：第一个元素是发送端 -- 发送者，而第二个元素是接收端 -- 接收者。
        // 由于历史原因，tx 和 rx 通常作为 发送者（transmitter）和 接收者（receiver）的缩写，
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let val = String::from("hi");
            tx1.send(val).unwrap();
        });
        let tx2 = tx.clone();
        thread::spawn(move || {
            let val = String::from("hi 2222222");
            tx2.send(val.clone()).unwrap();
            println!("val is {}", val);
        });

        // 在主线程中接收并打印内容 “hi”
        let received = rx.recv().unwrap();
        println!("Got: {}", received);

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    #[test]
    fn channel_2() {
        // mpsc::channel 函数返回一个元组：第一个元素是发送端 -- 发送者，而第二个元素是接收端 -- 接收者。
        // 由于历史原因，tx 和 rx 通常作为 发送者（transmitter）和 接收者（receiver）的缩写，
        let (tx, rx) = mpsc::channel();

        let val1 = String::from("hi111");
        tx.send(val1).unwrap();
        let val2 = String::from("hi2222");
        tx.send(val2.clone()).unwrap();
        let val3 = String::from("hi 3333");
        tx.send(val3).unwrap();

        // 在主线程中接收并打印内容 “hi”
        let received = rx.recv().unwrap();
        println!("channel_2 Got1: {}", received);
        let received = rx.recv().unwrap();
        println!("channel_2 Got2: {}", received);
        // let received = rx.recv().unwrap();
        // println!("channel_2 Got3: {}", received);
    }

    #[test]
    fn channel_3() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}