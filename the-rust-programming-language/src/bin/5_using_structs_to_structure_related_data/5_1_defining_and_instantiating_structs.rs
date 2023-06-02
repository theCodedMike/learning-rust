#![allow(dead_code)]
#![allow(unused_variables)]

/// 5.1 定义并举例说明结构体
/// cargo r --bin 5_1
fn main() {
    /*
    ## 定义并实例化结构体
    ### 变量与字段同名时的字段初始化简写语法
    ### 使用结构体更新语法从其他实例创建实例
    ### 使用没有命名字段的元组结构体来创建不同的类型
    ### 没有任何字段的类单元结构体
     */
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    let user4_copy = user4.clone();

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let i = 32;
    let i1 = i.clone();

    let s = String::from("djidjkdd");
    println!("{}", s);
}
// 结构体  这种最常见
#[derive(Clone)]
struct User {
    active: bool, // 字段(field)
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
fn build_user_simple_version(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// 单元结构体
struct AlwaysEqual;
