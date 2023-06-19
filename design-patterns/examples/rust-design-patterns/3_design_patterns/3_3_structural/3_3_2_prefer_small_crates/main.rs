//! ## 偏向更小的库
//!
//! ### 描述
//! 偏向于使用专注于做好一件事的包(crate)
//!
//! ### 优点
//! - 小的包更容易理解，并且鼓励更加模块化的代码
//! - 可以提高项目间代码的重用
//! - 由于Rust是以包为编译单元，把整个项目拆分为多个包可以使代码并行构建
//!
//! ### 缺点
//! - 当一个项目依赖多个有矛盾版本的包时，会导致"依赖地狱"
//! - crates.io里的包可能质量不佳
//! - 两个小包可能比一个大的包的优化要更少，因为编译器默认没有开启链接时优化
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_10
//!

fn main() {
    // The url crate provides tools for working with URLs.

    // The num_cpus crate provides a function to query the number of CPUs on a machine.

    // The ref_slice crate provides functions for converting &T to &[T]. (Historical example)
}
