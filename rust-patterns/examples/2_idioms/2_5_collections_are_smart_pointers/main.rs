//! ## 将集合视为智能指针
//!
//! ### 描述
//!
//! 使用集合的Deref特性使其像智能指针一样，提供数据的借用或者所有权
//!
//! ```rust
//! use std::ops::Deref;
//!
//! struct Vec<T> {
//!     data: T,
//!     //..
//! }
//!
//! impl<T> Deref for Vec<T> {
//!     type Target = [T];
//!
//!     fn deref(&self) -> &[T] {
//!         //..
//!     }
//! }
//! ```
//! ### 优点
//! 大部分方法可以只针对借用类型实现，这些实现对自有数据的类型可以隐式地适用
//!
//! ### 缺点
//! 边界检查时，不考虑仅通过解引用可用的方法和特性，所以对泛型数据结构使用这种模式将会变得复杂   
//! （请看 Borrow和AsRef特性）
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_5
//!
fn main() {
    println!("collections are smart pointers");
}
