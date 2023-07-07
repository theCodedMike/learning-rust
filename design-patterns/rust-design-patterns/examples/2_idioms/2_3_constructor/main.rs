use crate::constructor::{Country, Second};

mod constructor;
//! ## 构造器
//!
//! ### 描述
//! Rust 没有语言层面的构造器。 取而代之的是常用一个关联函数 new() 创建对象
//!
//! Rust支持通过使用Default特征来实现默认构造器
//!
//! 也可以通过derive派生宏(Default)来实现默认构造器
//!
//! ### 执行
//! cargo r --example 2_3
//!

fn main() {
    let second1 = Second::new(10);
    println!("{:?}, value is {}", second1, second1.value());
    // Second { value: 10 }, value is 10

    let second2 = Second::default();
    println!("{:?}, value is {}", second2, second2.value());
    // Second { value: 100 }, value is 100

    let country = Country::default();
    println!(
        "{:?}, name: {}, code: {}",
        country,
        country.get_name(),
        country.get_code()
    );
    // Country { name: "", code: 0 }, name: , code: 0
}
