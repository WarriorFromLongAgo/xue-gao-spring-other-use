#[cfg(test)]
mod thread_test_1 {
    use std::sync::{Arc, Mutex};
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

    #[test]
    fn thread_test_2() {
        for i in 1..10 {
            thread::spawn(move || {
                println!("hi number {} from the spawned my_thread!", i);
            });
        }
        for i in 1..10 {
            println!("hi number {} from the main my_thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn thread_test_no_join() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn thread_test_join() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    #[test]
    fn thread_test_mutex() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }

    #[test]
    fn thread_test_mutex_2() {
        // 方案 1
        // let counter = Mutex::new(0);
        // let mut handles = vec![];
        //
        // for _ in 0..10 {
        //     let countrt_i = counter.clone();
        //     let handle = thread::spawn(move || {
        //         let mut num = countrt_i.lock().unwrap();
        //
        //         *num += 1;
        //     });
        //     handles.push(handle);
        // }
        //
        // for handle in handles {
        //     handle.join().unwrap();
        // }
        //
        // println!("Result: {}", *counter.lock().unwrap());

        // 方案 2
        // let counter = Rc::new(Mutex::new(0));
        // let mut handles = vec![];
        //
        // for _ in 0..10 {
        //     let counter = Rc::clone(&counter);
        //     let handle = thread::spawn(move || {
        //         let mut num = counter.lock().unwrap();
        //
        //         *num += 1;
        //     });
        //     handles.push(handle);
        // }
        //
        // for handle in handles {
        //     handle.join().unwrap();
        // }
        //
        // println!("Result: {}", *counter.lock().unwrap());

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                println!("thread::spawn {}", i);
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            println!("handle.join().unwrap(): ");
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}