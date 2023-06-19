//! ## 透镜和棱镜
//!
//! ### 描述
//! 透镜和棱镜都是纯函数式编程语言里的概念
//!
//! 透镜(lens): 以抽象、统一的方式访问数据类型的部分字段
//! lens-rs是一个好的例子
//!
//! 棱镜(prism): A prism is a function that operates on a family of lenses
//! serde是一个好的例子
//!
//! ### 执行
//! cargo r --example 5_3
//!

mod lens;
mod lens_rs_crate;
mod prism;

fn main() {}
