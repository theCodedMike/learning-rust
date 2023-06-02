#![allow(dead_code)]
#![allow(unused_variables)]

/// 5.2 使用结构体的代码示例
/// cargo r --bin 5_2
fn main() {
    /*
    计算长方形面积
     */
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    //使用元组重构
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_use_tuple(rect1)
    );

    //使用结构体重构：赋予更多意义
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_use_struct(&rect1)
    );

    //通过派生 trait 增加实用功能
    println!("rect1 is {:?}", rect1); // Rectangle { width: 30, height: 50 }
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    // &rect1 = Rectangle {
    //     width: 60,
    //     height: 50,
    // }
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area_use_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area_use_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
