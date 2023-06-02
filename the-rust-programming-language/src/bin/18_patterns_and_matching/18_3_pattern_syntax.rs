#![allow(dead_code)]
#![allow(unused_variables)]

/// 18.3 模式语法
///
/// cargo r --bin 18_3
///
/// ## 目录:
/// ### 匹配字面量
/// ### 匹配命名变量
/// ### 多个模式
/// ### 通过 ..= 匹配值的范围
/// ### 解构并分解值
/// #### 解构结构体
/// #### 解构枚举
/// #### 解构嵌套的结构体和枚举
/// #### 解构结构体和元组
/// ### 忽略模式中的值
/// #### 使用 _ 忽略整个值
/// #### 使用嵌套的 _ 忽略部分值
/// #### 通过在名字前以一个下划线开头来忽略未使用的变量
/// #### 用 .. 忽略剩余值
/// ### 匹配守卫提供的额外条件
/// - 匹配守卫（match guard）: 是一个指定于 match 分支模式之后的额外 if 条件
/// ### @ 绑定
/// - at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
///
fn main() {
    // 匹配字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 5
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 多个模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"), // one or two
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 通过 ..= 匹配值的范围
    let x = 5;
    match x {
        1..=5 => println!("one through five"), // one through five
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"), // early ASCII letter
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构并分解值
    // 解构结构体
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 10 };
    match p {
        Point { x, y: 10 } => println!("On the x axis at {}", x), // 0
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    println!("{:?}", p); // Point { x: 0, y: 10 }

    // 解构枚举
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
            // red 0, green 160, and blue 255
        }
    }

    // 解构嵌套的结构体和枚举
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ); // hue 0, saturation 160, and value 255
        }
        _ => (),
    }

    // 解构结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y); // feet: 3, inches: 10, x: 3, y: -10

    // 忽略模式中的值
    // 使用 _ 忽略整个值
    foo(3, 4);

    // 使用嵌套的 _ 忽略部分值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value"); // √
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value); // Some(5)

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth); // 2 8 32
        }
    }

    // 通过在名字前以一个下划线开头来忽略未使用的变量
    let _x = 5;

    // 用 .. 忽略剩余值
    let origin = Point3 { x: 0, y: 0, z: 0 };
    match origin {
        Point3 { x, .. } => println!("x is {}", x), // 0
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last); // 2 32
        }
    }
    /* // 编译报错
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second); // 4
        },
    }*/

    // 匹配守卫提供的额外条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // 4
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n), // 10
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y); // Some(10) 10

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"), // √
    }

    // @ 绑定
    let msg = Message3::Hello { id: 5 };
    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable); // 5
        }
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message3::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
enum Message3 {
    Hello { id: i32 },
}
