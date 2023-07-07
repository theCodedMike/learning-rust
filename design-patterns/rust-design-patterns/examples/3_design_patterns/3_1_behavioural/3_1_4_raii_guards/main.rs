//! ## RAII守卫
//!
//! ### 描述
//! RAII代表“资源获取即初始化”。该模式的本质是，资源的初始化在对象的构造函数中完
//! 成，以及确定性析构器。通过使用一个RAII对象作为一些资源的守卫，并且依赖类型系统确保访问始终要通
//! 过守卫对象，以此在Rust中扩展这种模式
//!
//! ### 优点
//! 防止使用未初始化资源和销毁后资源的错误
//!
//! ### 缺点
//!
//! ### 讨论
//! RAII是确保资源被合适地析构或确定的实用模式。
//! 在Rust中使用借用检查器静态地防止析构后发生使用资源的错误。
//!
//! 借用检查器的核心目标是确保对数据的引用不能超过数据的生命周期
//!
//! ### 执行
//! cargo r --example 3_1_4
//!

use crate::model::{baz, Foo, Mutex as CustomMutex};
use std::sync::Mutex;

mod model;

fn main() {
    let greet = String::from("hello");
    let mutex = Mutex::new(greet);
    let result = mutex.lock();
    let mut guard = result.unwrap();
    guard.push_str(" world"); // 核心就是通过守卫来操作对象，而不是直接操作对象
    println!("{}", guard); // hello world

    let foo = Foo {};
    let mutex = CustomMutex::new(foo);
    baz(mutex); // Foo: foo
}
