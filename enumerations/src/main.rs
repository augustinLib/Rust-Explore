use std::str;

fn main() {
    enums_value();
    enums_parameter(IpAddrKind::V4);
    enums_with_struct();
    enums_with_type();
    enums_with_type2();
    enums_with_impl();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn enums_value() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;
    println!("four: {:?}, six: {:?}", four, six);
}


fn enums_parameter(ip_type: IpAddrKind) {
    println!("ip_type: {:?}", ip_type);
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn enums_with_struct() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {:?}, loopback: {:?}", home, loopback);
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

fn enums_with_type() {
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enums_with_type2() {
    // u8, u8, u8, u8
    let home = IpAddr3::V4(127, 0, 0, 1);

    // String
    let loopback = IpAddr3::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);
}

#[derive(Debug)]
enum Message {
    // Quit값은 연관 데이터를 갖지 않는다.
    Quit,

    // Move값은 anonymous struct(익명 구조체)를 갖는다.
    Move { x: i32, y: i32 },

    // Write값은 1개의 String type 값을 갖는다.
    Write(String),

    // ChangeColor값은 3개의 i32 type 값을 갖는다.
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call: {:?}", self);
    }
}

fn enums_with_impl() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct