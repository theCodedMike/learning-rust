use std::{fs, io};

//! ## 在栈上动态分发
//!
//! ### 描述
//! 可以使用延迟条件初始化（deferred conditional initialization）来扩展生命周期
//!
//! !!!特征对象
//!
//! ### 优点
//! 不用在堆上申请任何空间。既不用初始化任何用不上的东西，也不用单态化全部代码，便可同时支持File和Stdin
//!
//! ### 缺点
//! 这样写代码比使用Box实现的版本需要更多活动部件（moving parts）
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_8
//!
fn main() -> Result<(), io::Error> {
    // 它们必须活的比 `readable`长, 因此先声明:
    let (mut stdin_read, mut file_read);
    let arg = "-";

    // We need to ascribe the type to get dynamic dispatch.
    let _readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };
    // Read from `readable` here.

    // We still need to ascribe the type for dynamic dispatch.
    let _readable: Box<dyn io::Read> = if arg == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(arg)?)
    };
    // Read from `readable` here.

    return Ok(());
}
