#![allow(dead_code)]
#![allow(unused_variables)]

/// 6.1 定义枚举
///
/// cargo r --bin enum
///
/// ## 目录
/// ### 枚举值
///
/// ### Option 枚举和其相对于空值的优势
/// pub enum Option<T> {
///     None, /// No value
///     Some(T), /// Some value of type `T`
/// }
///
fn main() {
    /* 枚举值 */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{} bytes", std::mem::size_of_val(&four)); // 1
    println!("{} bytes", std::mem::size_of_val(&six)); // 1

    //啰嗦版
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //精简版
    let home = StringIpAddr::V4(String::from("127.0.0.1"));
    let loopback = StringIpAddr::V6(String::from("::1"));
    println!("{:?}", home); // V4("127.0.0.1")

    //精简版2
    let home = NewIpAddr::V4(127, 0, 0, 1);
    let loopback = NewIpAddr::V6(String::from("::1"));
    println!("{:?}", home); // V4(127, 0, 0, 1)

    let message = Message::ChangeColor(1, 2, 3);
    message.call();
    println!();

    /* Option 枚举和其相对于空值的优势  Rust使用None表达空值(Null) */
    let have = Some(5);
    let none: Option<i32> = None;
    println!("{:?}, {:?}", have, none); // Some(5), None
    let new = plus_one(have);
    let none_plus_val = plus_one(none);
    println!("{:?}, {:?}", new, none_plus_val); // Some(6), None
}
enum IpAddrKind {
    V4, // 成员(variant)
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了
#[derive(Debug)]
enum StringIpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum NewIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit, // 类单元结构体
    Move { x: i32, y: i32 },
    Write(String),              // 元组结构体
    ChangeColor(i32, i32, i32), // 元组结构体
}
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
