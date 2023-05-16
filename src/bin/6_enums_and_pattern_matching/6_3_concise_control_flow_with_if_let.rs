/// 6.3 if let 简单控制流
/// cargo r --bin 6_3
fn main() {
    /*
    ## if let 简单控制流

     */
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("anything"),
    }

    //等价于
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("anything");
    }

    let mut count = 0;
    let coin = Coin::Nickel;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("now count is {}", count); // 1

    //等价于
    let mut count = 0;
    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("now count is {}", count); // 1
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
