#[cfg(test)]
mod enum_test_1 {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    enum IpAddrV2 {
        V4(String),
        V6(String),
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    #[derive(Debug)]
    enum IpAddrV3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr_V4 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }


    struct QuitMessage; // 类单元结构体
    struct MoveMessage {
        x: i32,
        y: i32,
    }

    struct WriteMessage(String); // 元组结构体
    struct ChangeColorMessage(i32, i32, i32); // 元组结构体




    #[test]
    fn enum_test_1() {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        println!("home {:?}", home);
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
        println!("loopback {:?}", loopback);

        let homeV2 = IpAddrV2::V4(String::from("127.0.0.1"));
        println!("home {:?}", homeV2);

        let loopbackV2 = IpAddrV2::V6(String::from("::1"));
        println!("loopback {:?}", loopbackV2);

        let homeV3 = IpAddrV3::V4(127, 0, 0, 1);
        println!("home {:?}", homeV3);
        let loopbackV3 = IpAddrV3::V6(String::from("::1"));
        println!("loopback {:?}", loopbackV3);

        let m = Message::Write(String::from("hello"));
        m.call();
        
    }
}