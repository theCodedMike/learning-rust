//! ## 策略模式
//!
//! ### 描述
//! 策略模式是支持关注点分离的一门技术。 它还支持通过依赖倒置来分离软件模块
//!
//! 策略模式背后的基本思想是，给定一个解决特定问题的算法，我们仅在抽象层次上定义算法的框架，并将
//! 指定的算法实现分成不同的部分
//!
//! 使用该算法的客户端可以选择特定的实现，而通用的算法工作流可以保持不变。换句话说，类的抽
//! 象规范不依赖于派生类的具体实现，而是具体实现必须遵循抽象规范
//!
//! ### 优点
//! 主要的优点是分离关注点
//!
//! ### 缺点
//! 对于每个策略，必须至少实现一个模块，因此模块的数量会随着策略数量增加
//!
//! 如果有很多策略可供选择，那么用户就必须知道策略之间的区别
//!
//! ### 讨论
//! Serde库是策略模式的一个实践的好例子
//!
//! ### 执行
//! cargo r --example 3_1_5
//!

use crate::model::{Adder, Json, Report, Text};

mod model;

fn main() {
    let mut s = String::from("");
    Report::generate(Text, &mut s);
    println!("{}", s);
    // one 1
    // two 2

    s.clear(); // reuse the same buffer
    Report::generate(Json, &mut s);
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
    println!("{}", s);
    // [{"one":"1"},{"two":"2"}]

    // 用Rust的闭包来实现策略模式
    let arith_adder = |x, y| x + y;
    let bool_adder = |x, y| {
        if x == 1 || y == 1 {
            1
        } else {
            0
        }
    };
    let custom_adder = |x, y| 2 * x + y;

    assert_eq!(9, Adder::add(4, 5, arith_adder));
    assert_eq!(0, Adder::add(0, 0, bool_adder));
    assert_eq!(5, Adder::add(1, 3, custom_adder));

    let val = Some("Rust");
    let len_strategy = |s: &str| s.len();
    assert_eq!(4, val.map(len_strategy).unwrap());

    let first_byte_strategy = |s: &str| s.bytes().next().unwrap();
    assert_eq!(82, val.map(first_byte_strategy).unwrap());
}
