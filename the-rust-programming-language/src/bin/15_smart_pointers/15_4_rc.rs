#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

/// 15.4 Rc<T> 引用计数智能指针
///
/// cargo r --bin 15_4
///
/// ## 目录:
/// - 引用计数（reference counting）: 意味着记录一个值引用的数量来知晓这个值是否仍在被使用
/// - Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候
/// - Rc<T> 只能用于单线程场景
/// ### 使用 Rc<T> 共享数据
/// 问题引入，具体见assets/multi_to_one.png，此时使用Box无法解决
///
/// ### 克隆 Rc<T> 会增加引用计数
/// - 使用 Rc<T> 允许一个值有多个所有者，且多个所有者只能共享数据
///
fn main() {
    // 3|b ------->
    //            |
    //           5|a -------> 10|d -------> Nil
    //            |
    // 4|c ------>

    // 使用 Rc<T> 共享数据
    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    let b = List::Cons(3, Box::new(a));
    //let c = List::Cons(4, Box::new(a));
    //编译报错，a的所有权已被转移到b中。所以Box无法解决共享数据问题

    let a = Rc::new(List2::Cons(
        5,
        Rc::new(List2::Cons(10, Rc::new(List2::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = List2::Cons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 2
    let c = List2::Cons(4, Rc::clone(&a));
    println!("count after creating a = {}\n", Rc::strong_count(&a)); // 3

    // 克隆 Rc<T> 会增加引用计数
    let a = Rc::new(List2::Cons(
        5,
        Rc::new(List2::Cons(10, Rc::new(List2::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = List2::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = List2::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!(
        "count after c goes out of scope = {}\n",
        Rc::strong_count(&a)
    ); // 2
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}
