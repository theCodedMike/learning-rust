#![allow(dead_code)]
#![allow(unused_variables)]

/// 3.1 变量和可变性
///
/// cargo r --bin 3_1
///
/// ## 目录
/// ### 常量
/// ### 遮蔽
///
fn main() {
    // 变量和可变性
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 遮蔽
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}
