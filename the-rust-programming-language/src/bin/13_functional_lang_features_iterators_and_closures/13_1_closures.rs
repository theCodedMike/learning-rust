#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;
use std::time::Duration;

/// 13.1 闭包：可以捕获环境的匿名函数
///
/// cargo r --bin closure
///
/// ## 目录:
/// - 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数
/// - 不同于函数，闭包允许捕获调用者作用域中的值
/// ### 使用闭包创建行为的抽象
/// #### 使用函数重构
/// #### 重构使用闭包储存代码
/// ### 闭包类型推断和标注
/// - 闭包不要求像 fn 函数那样在参数和返回值上注明类型
/// ### 使用带有泛型和 Fn trait 的闭包
/// - 所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
/// - 惰性求值（memoization 或 lazy evaluation）：创建一个存放闭包和调用闭包结果的结构体,该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值
/// ### Cache 实现的限制
/// ### 闭包会捕获其环境
/// - 当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开销
/// - 闭包可以通过三种方式捕获其环境
/// ```
/// FnOnce: 获取其所有权并在定义闭包时将其移动进闭包，只能被调用一次
/// FnMut:  获取可变的借用值
/// Fn: 获取不可变的借用值
/// ```
/// - 所有闭包都实现了 FnOnce, 所有闭包都可以被调用至少一次
/// - 在参数列表前使用 move 关键字可以强制闭包获取其使用的环境值的所有权
fn main() {
    // 使用闭包创建行为的抽象
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    //generate_workout_1(
    //generate_workout_2(
    //generate_workout_3(
    /*    simulated_user_specified_value,
        simulated_random_number
    );*/

    // 闭包类型推断和标注
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;

    let example_closure = |x: String| x;
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5); //编译报错 expected struct `String`, found integer

    // 使用带有泛型和 Fn trait 的闭包
    //generate_workout(simulated_user_specified_value, simulated_random_number);

    // 闭包会捕获其环境，而函数无法做到
    let x = 4;
    let equal_to_x = |z| z == x; // 这里捕获了x
    let y = 4;
    assert_eq!(equal_to_x(y), true);
    /*
    let x = 4;
    fn equal_to_x(z: i32) -> bool { z == x }
    let y = 4;
    assert_eq!(equal_to_x(y), true);
    */
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    //println!("can't use x here: {:?}", x); // value borrowed here after move
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
fn generate_workout_1(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
// 使用函数重构
fn generate_workout_2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
// 使用闭包重构
fn generate_workout_3(intensity: u32, random_number: u32) {
    let expensive_closure = |num| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // 这里还是有点问题，因为会调用闭包多次，和用函数的效果类似，需要改进
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}
/// 惰性求值
///
/// 创建一个存放闭包和调用闭包结果的结构体
///
/// 该结构体只会在需要结果时执行闭包，并会缓存结果值
///
/// 这样余下的代码就不必再负责保存结果并可以复用该值
struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    operation: T,
    result: Option<u32>, //这里的局限性在于只能存1个值，可以考虑优化为map，这样就能存多个值了
}
impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(operation: T) -> Self {
        Cache {
            operation,
            result: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.result {
            None => {
                let value = (self.operation)(arg);
                self.result = Some(value);
                value
            }
            Some(old) => old,
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cache::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
