//! ## 用format!拼接字符串
//!
//! ### 描述
//! 拼接字符串，除了可以使用String的push或push_str方法或'+'操作符，还可以使用format!宏
//!
//! ### 优点
//! 使用format! 连接字符串通常更加简洁和易于阅读
//!
//! ### 缺点
//! 不是最有效的连接字符串的方法   
//! 对一个可变的String类型对象进行一连串的push操作通常是最有效率的
//!
//! ### 执行
//! cargo r --example 2_2
//!

fn main() {
    say_hello("world");
}

fn say_hello(name: &str) {
    // 我们可以手动构建字符串
    let mut result = "Hello ".to_owned();
    result.push_str(name);
    result.push('!');
    println!("{}", result); // Hello world!

    // 但是用format! 更好
    println!("{}", format!("Hello {}!", name)); // Hello world!
}
