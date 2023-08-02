#![allow(dead_code)]
#![allow(unused_variables)]

/// 6.3 if let 简单控制流
///
/// cargo r --bin if-let
///
fn main() {
    let some_u8_value = Some(3_u8);
    match some_u8_value {
        Some(val) => println!("val: {}", val), // 3
        _ => println!("anything"),             // 这里用None也可以
    }

    // 等价于
    if let Some(val) = some_u8_value {
        println!("val: {}", val); // 3
    } else {
        // 如果some_u8_value为None, 则会进入这个分支
        println!("anything");
    }

    let mut count = 0;
    let coin = Coin::Nickel;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("now count is {}", count); // 1

    // 等价于
    let mut count = 0;
    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("now count is {}", count); // 1
    println!();

    let none_val: Option<i32> = None;
    if let None = none_val {
        println!("Here is None"); // Here is None
    }

    let some_val = Some("rust");
    if let Some(val) = some_val {
        println!("Here is Some({})", val); // Here is Some(rust)
    }
}
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
