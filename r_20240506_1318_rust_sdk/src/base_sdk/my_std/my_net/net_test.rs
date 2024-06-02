#[cfg(test)]
mod net_test {
    use std::{fs, thread};
    use std::io::{BufRead, BufReader, Write};
    use std::net::{TcpListener, TcpStream};

    use futures::executor::ThreadPool;

    // cargo test -- --nocapture
    #[test]
    fn test_1() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
        }
    }

    #[test]
    fn test_2() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
            println!("Connection established!");
        }
    }


    fn handle_connection_1(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        if request_line == "GET / HTTP/1.1" {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("hello.html").unwrap();
            let length = contents.len();

            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents = fs::read_to_string("404.html").unwrap();
            let length = contents.len();

            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
    }

    #[test]
    fn test_3() {
        // 使用线程池改善吞吐量
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            // thread::spawn 会创建一个新线程并在其中运行闭包中的代码。
            thread::spawn(|| {
                handle_connection(stream);
            });
        }

        // 创建有限数量的线程
        // let pool = ThreadPool::new(4);
        // for stream in listener.incoming() {
        //     let stream = stream.unwrap();
        //     // thread::spawn 会创建一个新线程并在其中运行闭包中的代码。
        //     thread::spawn(|| {
        //         handle_connection(stream);
        //     });
        // }
    }

    // pub struct ThreadPool;
    //
    // impl ThreadPool {
    //     pub fn new(size: usize) -> ThreadPool {
    //         ThreadPool
    //     }
    // }
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     where
    //         F: FnOnce() -> T,
    //         F: Send + 'static,
    //         T: Send + 'static,
    // }

}