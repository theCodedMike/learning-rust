#![allow(dead_code)]
#![allow(unused_variables)]

/// 18.1 所有可能会用到模式的位置
///
/// cargo r --bin 18_1
///
/// ## 目录:
/// ### match 分支
/// ```
/// match VALUE {
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
/// }
/// ```
/// ### if let 条件表达式
/// ### while let 条件循环
/// ### for 循环
/// ### let 语句
/// ```
/// let PATTERN = EXPRESSION;
/// ```
/// ### 函数参数
///
fn main() {
    // match 分支
    let value = Some(4);
    match value {
        None => {
            println!("None");
        }
        Some(v) => {
            println!("value is {}", v);
        }
    }

    // if let 条件表达式
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color"); // √
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let 条件循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for 循环
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let 语句
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    let (x, y, _) = (1, 2, 3);

    // 函数参数
    fn foo(x: i32) {
        // 代码
    }
    let point = (3, 5);
    print_coordinates(&point);
}
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
