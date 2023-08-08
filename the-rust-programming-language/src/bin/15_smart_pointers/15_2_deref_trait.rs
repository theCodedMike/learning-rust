use std::ops::{Deref, DerefMut};

/// 15.2 通过 Deref trait 将智能指针当作常规引用处理
///
/// cargo r --bin deref
///
/// ## 目录
/// ### 追踪指针的值
/// - 解引用运算符(dereference operator): *
///
/// ### 像引用一样使用 Box<T>
///
/// ### 自定义智能指针
///
/// ### 通过实现 Deref trait 将某类型像引用一样处理
///
/// ### 函数和方法的隐式Deref强制转换
/// - 解引用强制转换（deref coercions）是 Rust 在函数或方法传参上的一种便利
/// - 解引用强制转换只能工作在实现了 Deref trait 的类型上
///
/// ### Deref强制转换如何与可变性交互
/// Rust 在发现类型和 trait 的实现满足以下三种情况时会进行解引用强制转换：
/// - 当 T: Deref<Target=U>      从 &T 到 &U
/// - 当 T: DerefMut<Target=U>   从 &mut T 到 &mut U
/// - 当 T: Deref<Target=U>      从 &mut T 到 &U
///
fn main() {
    /* 追踪指针的值 */
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // assert_eq!(5, y); //编译报错 can't compare `{integer}` with `&{integer}`
    assert_eq!(5, *y); // 解引用

    /* 像引用一样使用 Box<T> */
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    // assert_eq!(5, y); //编译报错 can't compare `{integer}` with `Box<{integer}>`
    assert_eq!(5, *y);

    /* 自定义智能指针 */
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);

    /* 通过实现 Deref trait 将某类型像引用一样处理 */
    assert_eq!(5, *y); // *(my_box.deref())

    /* 函数和方法的隐式解引用强制转换 */
    let m = MyBox::new(String::from("Rust"));
    //hello(&(*m)[..]); // 很啰嗦
    hello(&m); // &MyBox<String> -> &String -> &str
    hello(&&m); // &&MyBox<String> -> &MyBox<String> -> &String -> &str
    hello(&&&m); // &&&MyBox<String> -> &&MyBox<String> -> &MyBox<String> -> &String -> &str

    /* 解引用强制转换如何与可变性交互 */
    // Deref
    // DerefMut
}
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
