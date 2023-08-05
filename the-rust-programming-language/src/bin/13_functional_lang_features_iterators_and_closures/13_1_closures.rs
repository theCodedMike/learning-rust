#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Duration;

/// 13.1 闭包：可以捕获环境的匿名函数
///
/// cargo r --bin closure
///
/// ## 目录:
/// - 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数
/// - 不同于函数，闭包允许捕获调用者作用域中的值
/// ### 闭包会捕获其环境
/// - 当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开销
/// - 在参数列表前使用 move 关键字可以强制闭包获取其使用的环境值的所有权
///
/// ### 闭包类型推断和注解
/// - 闭包不要求像 fn 函数那样在参数和返回值上注明类型
/// - 闭包通常很短，并只关联于小范围的上下文而非任意情境
/// - 类似于变量，如果我们希望增加明确性和清晰度也可以添加类型标注，坏处是使代码变得更啰嗦
///
/// ### 捕获引用或者移动所有权
/// - 闭包可以通过三种方式捕获其环境，它们直接对应到函数获取参数的三种方式:不可变借用，可变借用和获取所有权
///
/// ### 将被捕获的值移出闭包和 Fn trait
/// - 闭包体可以做以下任何事: 将一个捕获的值移出闭包，修改捕获的值，既不移动也不修改值，或者一开始就不从环境中捕获值
/// - 闭包捕获和处理环境中的值的方式影响闭包实现的 trait. Trait 是函数和结构体指定它们能用的闭包的类型的方式.
/// - FnOnce、FnMut、Fn是3个**Trait**
/// - FnOnce 适用于能被调用一次的闭包，所有闭包都至少实现了这个 trait
/// - FnMut 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值。这类闭包可以被调用多次
/// - Fn 适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包, 当然也包括不从环境中捕获值的闭包. 这类闭包可以被调用多次而不改变它们的环境
///
///          闭包被调用的次数   将捕获的值移出闭包   修改被捕获的值   既不将被捕获的值移出闭包体也不修改被捕获
///  FnOnce     <= 1               √                x                 x
///   FnMut     >= 1               x                √                 x
///      Fn     >= 1               x                x                 √
///
/// - 3种Trait的依赖关系:
/// ```rust
/// pub trait FnOnce<Args> {
///     type Output;
///     fn call_once(self, args: Args) -> Self::Output;
/// }
/// pub trait FnMut<Args> : FnOnce<Args> {
///     fn call_mut(&mut self, args: Args) -> Self::Output;
/// }
/// pub trait Fn<Args> : FnMut<Args> {
///     fn call(&self, args: Args) -> Self::Output;
/// }
/// ```
///
/// ### 使用带有泛型和 Fn trait 的闭包
/// - 所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
/// - 惰性求值（memoization 或 lazy evaluation）：创建一个存放闭包和调用闭包结果的结构体,该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值
///
/// ### Cache 实现的限制
///
fn main() {
    /* 闭包会捕获其环境 */
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    ); // The user with preference Some(Red) gets Red

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    ); // The user with preference None gets Blue
    println!();

    /* 闭包类型推断和标注 */
    // 函数版本
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    // 闭包版本
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x| x + 1; // 没有标注类型
    println!("add_one_v4: {:?}", add_one_v4(3_i32)); // 4
    let example_closure = |x| x; // fn(String) 无返回值
    let example_closure2 = |x: String| x; // fn(String) -> String  有返回值
    let x = example_closure(String::from("hello")); // 第一次使用闭包后自动推断出x为String类型
                                                    //example_closure(5); //编译报错 expected struct `String`, found integer
    let x = example_closure2(String::from("hello"));
    println!("x is {}", x);
    println!();

    /* 捕获引用或者移动所有权 */
    // 捕获不可变引用的闭包
    let list = vec![1, 2, 3];
    println!("Before defining closure(ref): {:?}", list); // [1, 2, 3]
    let only_borrows = || println!("           From closure: {:?}", list);
    println!(" Before calling closure: {:?}", list); // [1, 2, 3]
    only_borrows();
    println!("  After calling closure: {:?}", list); // [1, 2, 3]

    // 捕获可变引用的闭包
    let mut list = vec![1, 2, 3];
    println!("Before defining closure(ref mut): {:?}", list); // [1, 2, 3]
    let mut borrows_mutably = || list.push(7); // 注意这行中必须要有mut才能调用闭包
    borrows_mutably();
    println!("After calling closure: {:?}", list); // [1, 2, 3, 7]

    // 强制闭包获取它用到的环境中值的所有权  move
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    std::thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    //println!("After calling closure: {:?}", list); // 编译报错, value borrowed here after move
    println!();

    /* 将被捕获的值移出闭包和 Fn trait */
    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }
    let mut list = [
        Rectangle::new(10, 1),
        Rectangle::new(3, 5),
        Rectangle::new(7, 12),
    ];
    let fn_mut_closure1 = |r: &Rectangle| r.width; // 这个闭包实现了FnMut
    let mut sort_operations = vec![];
    let value = String::from("by key called");
    let fn_once_closure = |r: &Rectangle| {
        // 这个闭包只实现了FnOnce
        sort_operations.push(value);
        r.width
    };
    let mut num_sort_operations = 0;
    let fn_mut_closure2 = |r: &Rectangle| {
        // 这个闭包实现了FnMut
        num_sort_operations += 1;
        r.width
    };
    list.sort_by_key(fn_mut_closure2);
    // [Rectangle { width: 3, height: 5 }, Rectangle { width: 7, height: 12 }, Rectangle { width: 10, height: 1 }]
    println!("{:?}", list);
    println!();

    /* 补充 */
    /* 使用带有泛型和 Fn trait 的闭包 */
    generate_workout(5, 6);
    // 一个闭包可以同时实现多个Trait(Fn FnMut FnOnce)
    let double = |x| x * 2;
    println!("{}", call_with_fn(double)); // 2
    println!("{}", call_with_fn_mut(double)); // 2
    println!("{}", call_with_fn_once(double)); // 2
    println!();

    /* 闭包作为函数返回值 */
    let as_return = closure_as_return();
    println!("closure as return: {} {}", as_return(4), as_return(5)); // 9 10
    let as_return2 = closure_as_return2(-1);
    println!(
        "closure as return: {:?} {:?}",
        as_return2(10),
        as_return2(11)
    ); // 5 6
    println!();

    /* 闭包的生命周期 */
    //let closure_elision = |x: &i32| -> &i32 { x }; //无法通过编译  要求入参的生命周期长于出参的，无解
    let new_closure = convert(|x: &i32| -> &i32 { x }); //解绝方法之一
    println!("closure lifetime elision: {}", new_closure(&5)); // 5
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
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
        std::thread::sleep(Duration::from_secs(1));
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

fn call_with_fn<F>(func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    func(1)
}
fn call_with_fn_mut<F>(mut func: F) -> usize
where
    F: FnMut(usize) -> usize,
{
    func(1)
}
fn call_with_fn_once<F>(func: F) -> usize
where
    F: FnOnce(usize) -> usize,
{
    func(1)
}

fn closure_as_return() -> impl Fn(i32) -> i32 {
    let num = 5;
    move |x| x + num
}
fn closure_as_return2(i: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    if i > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}

fn convert<T, F>(f: F) -> F
where
    F: Fn(&T) -> &T,
{
    f
}
