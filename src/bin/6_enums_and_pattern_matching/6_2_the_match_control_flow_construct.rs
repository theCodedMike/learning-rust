/// 6.2 match控制流运算符
/// cargo r --bin 6_2
fn main() {
    /*
    ## match 控制流运算符
    ### 绑定值的模式

    ## 匹配 Option<T>
    ### 匹配 Some(T)

    ## 匹配是穷尽的（exhaustive）

    ## 通配模式和 _ 占位符
     */
    println!("{}", value_in_cents(Coin::Dime)); // 10

    //绑定值的模式
    //匹配 Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //通配模式和 _ 占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => roll(),
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
