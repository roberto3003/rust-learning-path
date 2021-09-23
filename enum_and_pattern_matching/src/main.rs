//enum IpAddrKind {
//    V4,
//    V6,
//}

//enum IpAddr {
//    V4(String),
//    V6(String),
//}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// fn route(IpAddrKind::V4);
// fn route(IpAddrKind::V6);

//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

fn main() {
    //let home = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from(
    //        "127.0.0.1"
    //    ),
    //};
    //let loopback = IpAddr {
    //    kind: IpAddrKind::V6,
    //    address: String::from("::1"),
    //};

    //    let four = IpAddrKind::V4;
    //    let six = IpAddrKind::V6;

    //    let home = IpAddr::V4(String::from("127.0.0.1"));
    //    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);

    //    let loopback = IpAddr::V6(String::from("::1"));
    let loopback = IpAddr::V6("::1".to_owned());

    //    impl Message {
    //        fn call(&self) {
    //            // method
    //        }
    //    }

    //    let m = Message::Write("hello".to_owned());
    //    m.call();
}
