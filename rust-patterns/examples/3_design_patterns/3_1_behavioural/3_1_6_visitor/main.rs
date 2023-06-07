//! ## 访问者
//!
//! ### 描述
//! 访问者封装了在不同对象集合上运行的算法。它支持在不修改数据的情况下，支持不同算法。
//!
//! ### 优点
//!
//! ### 缺点
//!
//! ### 讨论
//! visit_*通常返回空值（与示例中的相反）
//!
//! ### 执行
//! cargo r --example 3_1_6
//!

mod model;

use crate::model::{walk_expr, Interpreter};
use model::ast::Expr;

fn main() {
    let mut interpreter = Interpreter;

    let left = Box::new(Expr::IntLit(10));
    let right = Box::new(Expr::IntLit(20));
    let res = walk_expr(&mut interpreter, &Expr::Add(left, right));
    println!("{}", res); // 30

    let left = Box::new(Expr::IntLit(5));
    let right = Box::new(Expr::IntLit(20));
    let res = walk_expr(&mut interpreter, &Expr::Sub(left, right));
    println!("{}", res); // -15
}
