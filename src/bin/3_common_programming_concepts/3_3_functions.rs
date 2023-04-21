/// 3.3 函数
///
/// cargo r --bin 3_3
///
/// ## 目录
/// ### 参数
/// ### 语句和表达式
/// ### 带有返回值的函数
///
fn main() {
    // 函数  snake_case
    println!("Hello, world!");
    another_function();

    // 参数
    another_function_with_parameter(5);
    print_labeled_measurement(5, 'h');

    // 语句和表达式
    /*
    语句（statement）是执行一些操作但不返回值的指令
    表达式（expression）计算并产生一个值
    函数调用是一个表达式。宏调用是一个表达式。大括号（代码块） {} 也是一个表达式
     */
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // 带有返回值的函数
    let x = five();
    println!("The value of x is: {}", x);
    let z = plus_one(5);
    println!("The value of z is: {}", z);

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