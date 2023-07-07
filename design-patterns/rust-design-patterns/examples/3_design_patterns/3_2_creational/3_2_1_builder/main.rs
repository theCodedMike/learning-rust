//! ## 通过调用建造者来构造对象
//!
//! ### 描述
//!
//! ### 优点
//!
//! ### 缺点
//! 与直接构造一个结构体或者一个简单的构造函数相比，这种方法太复杂
//!
//! ### 讨论
//! 由于一个方法一个名称不能重载，所以Rust相比于C++、Java来说更不适合写很多构造器
//!
//! ### 执行
//! cargo r --example 3_2_1
//!
//! 已经有三方包可以默认实现Builder模式，见
//! third-party-crates/src/bin/derive_builder.rs
//!

use crate::model::{Foo, FooBuilder};

mod model;

fn main() {
    let mut foo = Foo::new();
    foo.set_bar(String::from("Y"));

    let foo_from_builder: Foo = FooBuilder::new().name(String::from("Y")).build();

    assert_eq!(foo, foo_from_builder);
}
