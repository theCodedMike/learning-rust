#![allow(dead_code)]
#![allow(unused_variables)]

/// 5.3 方法语法
///
/// cargo r --bin method
///
/// ## 目录
/// ### 定义方法
///
/// ### 带有更多参数的方法
///
/// ### 关联函数(associated function)
///
/// ### 多个impl块
///
fn main() {
    /* 定义方法 */
    let rect1 = Rectangle::new(30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    ); // 1500
    println!();

    /* 带有更多参数的方法 */
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false
    println!();

    /* 关联函数 */
    let new_rec = Rectangle::new(15, 20);
    let square = Rectangle::square(34);
    println!("{:?}\n{:?}", new_rec, square);
    println!();

    /* 多个 impl 块 */
    //每个结构体都允许拥有多个impl块, 更多见的是每实现一个trait, 就定义一个impl块
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    /// 关联函数
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    /// 关联函数
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    /// 方法, 方法的第1个参数都是self, 而函数不以self为第一参数
    ///      self 即 self: Self
    ///     &self 即 self: &Self
    /// &mut self 即 self: &mut Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// 方法
    /// Getters  与字段同名的方法将被定义为只返回字段中的值
    fn width(&self) -> &u32 {
        &self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
