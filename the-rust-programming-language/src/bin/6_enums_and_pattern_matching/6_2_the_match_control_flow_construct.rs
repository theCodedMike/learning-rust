#![allow(dead_code)]
#![allow(unused_variables)]

/// 6.2 match控制流结构
///
/// cargo r --bin match
///
/// ## 目录
/// ### 绑定值的模式
///
/// ### 匹配Option<T>
/// #### 匹配Some(T)
///
/// ### 匹配是穷尽的
///
/// ### 通配模式和 _ 占位符
///
fn main() {
    /* 绑定值的模式 */
    println!("{}", value_in_cents(Coin::Dime)); // 10
    println!();

    /* 匹配 Option<T> */
    let five = Some(5);
    let six = plus_one(five); // Some(6)
    let none = plus_one(None); // None

    /* 通配模式和 _ 占位符 */
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => roll(), // _表示除了3和7以外的其他所有情况
    }
    println!();

    /* 补充 */
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo { x: 1, y: 2 },
        Action::ChangeColor(255, 255, 0),
    ];
    for action in &actions {
        match action {
            Action::Say(s) => {
                println!("        Action::Say {}", s); // Action::Say Hello Rust
            }
            Action::MoveTo { x: a, y: b } => {
                println!(
                    "     Action::MoveTo point from (0, 0) move to ({}, {})",
                    a, b
                ); // Action::MoveTo point from (0, 0) move to (1, 2)
            }
            Action::ChangeColor(r, g, _) => {
                println!("Action::ChangeColor change color into '(r:{}, g:{}, b:0)', 'b' has been ignored", r, g);
            } // Action::ChangeColor change color into '(r:255, g:255, b:0)', 'b' has been ignored
        }
    }
    println!();

    /* matches!宏 */
    let ch = 'f';
    println!("{}", matches!(ch, 'a'..='z' | 'A'..='Z')); // true
    let bar = Some(4);
    println!("{}", matches!(bar, Some(x) if x > 2)); // true
    println!("{}", matches!(bar, Some(x) if x > 10)); // false
    println!("{}", matches!(bar, None)); // false
    let a = 5;
    println!("{}", matches!(a, 5)); // true
    println!("{}", matches!(a, 6)); // false
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("now state is {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // 必须穷尽所有的情况
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("add_fancy_hat");
}

fn remove_fancy_hat() {
    println!("remove_fancy_hat");
}

fn roll() {
    println!("roll");
}

#[derive(Debug, PartialEq)]
enum Action {
    Say(String),
    MoveTo { x: i32, y: i32 },
    ChangeColor(u16, u16, u16),
}
