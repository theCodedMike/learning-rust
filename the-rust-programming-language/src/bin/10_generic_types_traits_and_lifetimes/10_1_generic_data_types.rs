#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

use num::ToPrimitive;
use std::fmt::{Debug, Display};

/// 10.1 泛型数据类型
///
/// cargo r --bin generic
///
/// ## 目录
/// 泛型分为2种:
///   - 针对类型的泛型(最常见的)  
///   - 针对值的泛型
/// ### 在函数定义中使用泛型
///
/// ### 结构体定义中的泛型
///
/// ### 枚举定义中的泛型
///
/// ### 方法定义中的泛型
///
/// ### 泛型代码的性能
/// - Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率，无性能损失
/// - 单态化： 将通用代码转换为特定代码
///
fn main() {
    /* 在函数定义中使用泛型 */
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result); // 100
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result); // y
    println!(
        "UseGeneric: The largest number is {}",
        largest(&number_list)
    ); // 100
    println!("UseGeneric: The largest char is {}", largest(&char_list)); // y
    println!();

    /* 结构体定义中的泛型 */
    let i32_point = Point::new(5, 6);
    i32_point.print(); // Point: x = 5, y = 6
    let str_point = Point::new(String::from("hello"), String::from("world"));
    str_point.print(); // Point: x = hello, y = world
    let line = Line::new("hello", 5);
    line.print(); // Line: x = hello, y = 5
    println!();

    /* 枚举定义中的泛型 */
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    let option: Option<i32> = Some(5);
    let result: Result<f64, ()> = Ok(3.2);

    /* 方法定义中的泛型 */
    let point = Point::new(10, 20);
    let x = point.x();
    println!("x is {}", x); // 10
    point.print_z("hello"); // Point: x = 10, y = 20, z = hello
    let mix_up = line.mix_up(Line::new(3.2, true));
    mix_up.print(); // Line: x = hello, y = true
    println!();

    /* 补充: const泛型 (Rust 1.51 版本引入的重要特性) 针对值的泛型 */
    // 问题引入
    let arr_3 = [1, 2, 3];
    display_array(arr_3);
    let arr_2 = [4, 6];
    // display_array(arr_2); // 无法编译，因为 mismatched types
    //解决方案1: 数组切片
    display_array_slice(&arr_3);
    display_array_slice(&arr_2);
    //解决方案2: 数组切片并使用泛型
    let arr_2 = [4.to_string(), 6.to_string()];
    display_infinite_array(&arr_3);
    display_infinite_array(&arr_2);
    //解决方案3: const泛型
    let arr_2 = [4.to_f64(), 6.to_f64()];
    display_array_final_v(arr_3);
    display_array_final_v(arr_2);
    println!();

    /* 同时使用针对类型的泛型和针对值的泛型 */
    let l = ArrayPair {
        left: ['0', '1', '2', '3', '4'],
        right: [9, 8, 7, 6, 5],
    };

    // const泛型参数目前只能使用以下形式的实参
    // 1: 一个单独的 const 泛型参数
    // 2: 一个字面量 (i.e. 整数, 布尔值或字符).
    // 3: 一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)
    bar(l.right);

    /* const 泛型表达式, 目前只能在nightly版本下使用 */
    something([0_u8; 0]);
    something([0_u8; 512]);
    //something([0_u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制
}
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// 这2种写法都可以
///
/// 等价于 fn largest<T: PartialOrd>(list: &[T]) -> &T {}
///
fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    if list.is_empty() {
        panic!("list can't be empty");
    }
    let mut largest = list.get(0).unwrap();
    for item in list.iter() {
        if *item > *largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}
/// 注意: impl后面必须有<T>，因为Point<T>是类型名称，并不能体现出该结构体要使用泛型，所以在impl后面加上<T>以体现该结构体要使用泛型
///
/// T: Display  这个叫特征约束，表明T这种泛型类型必须实现Display特征
///
impl<T: Display> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    fn print(&self) {
        println!("Point: x = {}, y = {}", self.x, self.y);
    }
    fn x(&self) -> &T {
        &self.x
    }
    fn print_z<Z: Display + Copy>(&self, z: Z) {
        println!("Point: x = {}, y = {}, z = {}", self.x, self.y, z);
    }
}

impl Point<f32> {
    /// 只有 T 为 f32 时，实例才能调用这个方法
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Line<T, U> {
    x: T,
    y: U,
}
impl<T, U> Line<T, U>
where
    T: Display,
    U: Display,
{
    fn new(x: T, y: U) -> Self {
        Line { x, y }
    }
    fn print(&self) {
        println!("Line: x = {}, y = {}", self.x, self.y);
    }
    fn mix_up<V, W>(self, other: Line<V, W>) -> Line<T, W> {
        Line {
            x: self.x,
            y: other.y,
        }
    }
}

fn display_array(arr: [i32; 3]) {
    println!("origin: {:?}", arr);
}
fn display_array_slice(arr: &[i32]) {
    println!("array slice: {:?}", arr);
}
fn display_infinite_array<T: Debug>(arr: &[T]) {
    println!("array slice with generics: {:?}", arr);
}
/// N 就是 const 泛型
fn display_array_final_v<T: Debug, const N: usize>(arr: [T; N]) {
    println!("const generics: {:?}", arr);
}

fn foo2<const N: usize>() {
    println!("foo2: N is {}", N);
}
fn bar<T, const M: usize>(data: [T; M]) {
    foo2::<M>(); //ok,符合第1种
    foo2::<2021>(); //ok,符合第2种
    foo2::<{ 30 * 100 + 20 * 10 + 1 }>(); //ok,符合第3种
                                          //foo2::<{ M + 1 }>(); //error,违背第3种,const表达式中不能有泛型参数M
                                          //foo2::<{ std::mem::size_of::<T>() }>(); //error，泛型表达式里不能含有泛型参数
    let _t: [u8; M]; //ok,符合第1种
                     //let _y: [u8; std::mem::size_of::<T>()]; //error，泛型表达式里不能含有泛型参数
}

//这里是一个 const 表达式 `core::mem::size_of::<T>() < 768`
fn something<T>(val: T)
// where Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    // do something
}

pub enum Assert<const CHECK: bool> {
    //
}

pub trait IsTrue {
    //
}

impl IsTrue for Assert<true> {
    //
}

struct ArrayPair<T1, T2, const N: usize> {
    left: [T1; N],
    right: [T2; N],
}
