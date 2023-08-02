#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

/// 3.3 函数
///
/// cargo r --bin fn
///
/// ## 目录
/// ### 参数
///
/// ### 语句和表达式
///     语句（statement）会执行一些操作但是不返回值
///     表达式（expression）会计算并产生一个值
///     函数调用是一个表达式。宏调用是一个表达式。大括号(代码块) {} 也是一个表达式
///
/// ### 带有返回值的函数
///
fn main() {
    /* 语句和表达式 */
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y); // 4
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let res = if x % 2 == 1 { "odd" } else { "even" };
    println!("x is {}", res); // odd
    println!("result of fn is: {}", add_with_extra(5, 6)); // 17
    println!();

    // 函数  snake_case
    another_function();

    // 参数
    another_function_with_parameter(5);
    print_labeled_measurement(5, 'h');

    // 带有返回值的函数 函数的返回值就是函数体最后一条表达式的返回值
    let x = five();
    println!("The value of x is: {}", x); // 5
    let z = plus_one(5);
    println!("The value of z is: {}", z); // 6

    //无返回值()

    //永不返回的发散函数 !
    //dead_end();
}
/// Rust的函数体是由一系列语句组成，最后由一个表达式来返回值
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; //语句
    let y = y + 5; //语句
    x + y //表达式
}

fn another_function() {
    println!("Another function.");
}
fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    //return x + 1;
    x + 1
}
/// 以下4种方式使得该函数的返回值为 !
fn dead_end() -> ! {
    panic!("it's time to panic");
    unimplemented!("I'm sorry");
    loop {}
    todo!("no time to work");
}
