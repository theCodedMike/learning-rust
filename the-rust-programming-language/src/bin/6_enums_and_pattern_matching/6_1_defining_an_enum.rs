#![allow(dead_code)]
#![allow(unused_variables)]

/// 6.1 定义枚举
/// cargo r --bin 6_1
fn main() {
    /*
    ## 定义枚举
    ### 枚举值
     */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

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

    //
    let home = NewIpAddr::V4(127, 0, 0, 1);
    let loopback = NewIpAddr::V6(String::from("::1"));

    let message = Message::ChangeColor(1, 2, 3);
    message.call();

    //Option 枚举和其相对于空值的优势
    let have = Some(5);
    let none: Option<i32> = None;
}
enum IpAddrKind {
    V4, // 成员（variants）
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
// 将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了
enum StringIpAddr {
    V4(String),
    V6(String),
}
enum NewIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
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
