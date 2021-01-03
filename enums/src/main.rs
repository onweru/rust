#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

// fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

// enum IpAddress {
//     V4(String),
//     V6(String)
// }

enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // method body
    }
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// enum Option<T> {
//     Some(T),
//     None
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // route(four);
    // route(six);

    let _home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1")
    };

    let _loopback = IpAddr {
        kind: six,
        address: String::from("::1")
    };

    // let home = IpAddress::V4(String::from("127.0.0.1"));
    let home = IpAddress::V4(127,0,0,1);
    let loopback = IpAddress::V6(String::from("::1"));

    println!("I have a IPv4 () & IPV6 () addresses");

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
}
