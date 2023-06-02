#![allow(dead_code)]
#![allow(unused_variables)]

/// 5.3 方法语法
/// cargo r --bin 5_3
fn main() {
    /*
    ## 方法语法
    ### 定义方法
     */
    //定义方法
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    ); // 1500

    //带有更多参数的方法
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true

    //关联函数
    let new_rec = Rectangle::new(15, 20);
    let square = Rectangle::square(34);
    println!("{:?}\n{:?}", new_rec, square);

    //多个 impl 块
    //每个结构体都允许拥有多个 impl 块
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
    //      self 即 self: Self
    //     &self 即 self: &Self
    // &mut self 即 self: &mut Self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Getters  与字段同名的方法将被定义为只返回字段中的值
    fn width(&self) -> &u32 {
        &self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
