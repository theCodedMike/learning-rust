#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;

/// 19.4 高级函数和闭包
///
/// cargo r --bin advanced-fn-and-closure
///
/// ## 目录
/// ### 函数指针
/// - fn 被称为 函数指针(function pointer)
/// - 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数
/// - 函数指针实现了所有三个闭包 trait(Fn、FnMut 和 FnOnce)
///
/// ### 返回闭包
///
fn main() {
    /* 函数指针 */
    let add_one_closure = |i: i32| -> i32 { i + 1 };
    let do_twice_closure = |clo: fn(i32) -> i32, i: i32| -> i32 { clo(i) + clo(i) };
    println!("use closure: {}", do_twice_closure(add_one_closure, 5)); // 12

    let answer = do_twice(add_one, 5);
    println!("use fn pointer: {}", answer); // 12

    let list_of_numbers = vec![1, 2, 3];
    let str_vec_use_closure = list_of_numbers
        .iter()
        .map(|i| i.to_string()) // 使用闭包
        .collect::<Vec<String>>();
    let str_vec_use_fn = list_of_numbers
        .iter()
        .map(ToString::to_string) // 使用函数指针
        .collect::<Vec<String>>();
    assert_eq!(str_vec_use_closure, str_vec_use_fn);

    let list_of_statuses = (0_u32..20).map(Status::Value).collect::<Vec<_>>();
    assert_eq!(list_of_statuses.len(), 20);
    println!("{:?}", list_of_statuses); // [Value(0), Value(1), Value(2), ... , Value(19)]
    println!();

    /* 返回闭包 */
    let closure = returns_closure();
    println!("{}", closure(5)); // 6
    let closure2 = returns_closure2();
    println!("{}", closure2(5)); // 6 or 10
}
///
/// 函数指针
///
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn returns_closure2() -> impl Fn(i32) -> i32 {
    let half = rand::thread_rng().gen_range(0..10) > 5;
    if half {
        |x| x + 1
    } else {
        |x| x + 5
    }
}
