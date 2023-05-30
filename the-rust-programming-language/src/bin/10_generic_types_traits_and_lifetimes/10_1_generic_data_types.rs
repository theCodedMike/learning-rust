use std::fmt::Display;

/// 10.1 泛型数据类型
///
/// cargo r --bin 10_1
///
/// # 目录:
/// ### 1、泛型数据类型
/// #### 1.1 在函数定义中使用泛型
/// #### 1.2 结构体定义中的泛型
/// #### 1.3 枚举定义中的泛型
/// #### 1.4 方法定义中的泛型
/// ### 2、泛型代码的性能
/// - Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率，无性能损失
/// - 单态化： 将通用代码转换为特定代码
fn main() {
    // 在函数定义中使用泛型
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

    // 结构体定义中的泛型
    let i32_point = Point::new(5, 6);
    i32_point.print(); // Point: x = 5, y = 6
    let str_point = Point::new(String::from("hello"), String::from("world"));
    str_point.print(); // Point: x = hello, y = world

    let line = Line::new("hello", 5);
    line.print(); // Line: x = hello, y = 5

    // 枚举定义中的泛型
    let option: Option<i32> = Some(5);
    let result: Result<f64, ()> = Ok(3.2);

    // 方法定义中的泛型
    let point = Point::new(10, 20);
    let x = point.x();
    println!("x is {}", x); // 10
    point.print_z("hello"); // Point: x = 10, y = 20, z = hello
    let mix_up = line.mix_up(Line::new(3.2, true));
    mix_up.print(); // Line: x = hello, y = true
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
// 这2种写法都可以
// fn largest<T: PartialOrd>(list: &[T]) -> &T {
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
// 注意: impl后面必须有<T>，因为Point<T>是类型名称，并不能体现出该结构体要使用泛型，所以在impl后面加上<T>以体现该结构体要使用泛型
// T: Display  这个叫特征约束，表明T这种泛型类型必须实现Display特征
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
// 只有 T 为 f32 时，实例才能调用这个方法
impl Point<f32> {
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
