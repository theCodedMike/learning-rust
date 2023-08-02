#![allow(dead_code)]
#![allow(unused_variables)]

/// 5.1 定义并举例说明结构体
///
/// cargo r --bin struct
///
/// ## 目录
/// ### 变量与字段同名时的字段初始化简写语法
///
/// ### 使用结构体更新语法从其他实例创建实例
/// .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值
///
/// ### 使用没有命名字段的元组结构体来创建不同的类型
///
/// ### 没有任何字段的类单元结构体
///
fn main() {
    /* 变量与字段同名时的字段初始化简写语法 */
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let email = String::from("someone@example.com");
    let username = String::from("someusername123");
    let mut user2 = User::build_user_simple_version(email, username);
    user2.email = String::from("anotheremail@example.com");

    /* 使用结构体更新语法从其他实例创建实例 */
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
    // println!("user2 is not valid here: {:?}", user2); // 无法通过编译，因为user2的部分所有权已move
    // 把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段
    // println!("user2.username = {}", user2.username);
    println!("user2.email = {}", user2.email); // anotheremail@example.com

    let user4_copy = user4.clone();
    println!("{:?}", user4_copy); // User { active: true, username: "someusername123", email: "another@example.com", sign_in_count: 1 }
    println!();

    /* 使用没有命名字段的元组结构体来创建不同的类型 */
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black); // Color(0, 0, 0)
    println!("Point: {} {} {}", origin.0, origin.1, origin.2); // Point: 0 0 0
    println!();

    /* 没有任何字段的类单元结构体 */
    let subject = AlwaysEqual;
    println!("{:?}", subject); // AlwaysEqual
    println!("{} bytes", std::mem::size_of_val(&subject)); // 0 bytes

    let x = AlwaysEqualLike {};
    println!("{} bytes", std::mem::size_of_val(&x)); // 0 bytes
    println!();

    /* 结构体数据的所有权 生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小 */
    // 这里user_ref的作用范围必须要<=email/username的作用范围
    let user_ref = UserRef {
        email: "19945@qq.com",
        username: "dachui",
        active: true,
        sign_in_count: 8,
    };
}

// 结构体  这种最常见
#[derive(Clone, Debug)]
struct User {
    active: bool, // 字段(field)
    username: String,
    email: String,
    sign_in_count: u64,
}
impl User {
    // 关联函数, 未使用字段初始化简写语法
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    // 关联函数, 使用字段初始化简写语法
    fn build_user_simple_version(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

#[derive(Debug)]
struct UserRef<'a> {
    username: &'a str,
    // expected named lifetime parameter
    email: &'a str,
    // expected named lifetime parameter
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 元组结构体 (tuple structs)
#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体 (unit-like structs)
// 这2种都可以，推荐第1种，更简洁
#[derive(Debug)]
struct AlwaysEqual; // 注意这里有';'
struct AlwaysEqualLike {} // 注意这里没有';'
