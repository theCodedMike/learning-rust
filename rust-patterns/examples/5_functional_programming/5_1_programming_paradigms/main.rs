//! ## 编程范式
//!
//! ### 命令式
//! 关注如何做
//!
//! ### 声明式
//! 关注做了什么
//!
//! ### 执行
//! cargo r --example 5_1
//!

fn main() {
    // 命令式
    let mut sum = 0;
    for i in 1..11 {
        sum += i;
    }
    println!("{}", sum); // 55

    // 声明式
    let sum = (1..11).fold(0, |a, b| a + b);
    println!("{}", sum); // 55
}
