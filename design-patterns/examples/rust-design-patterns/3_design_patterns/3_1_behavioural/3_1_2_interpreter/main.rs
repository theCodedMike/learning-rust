//! ## 解释器
//! 如果一个问题经常出现并且需要很多且重复的步骤来解决，那么问题应该被抽象为一个简单的语言并且一
//! 个解释器对象能通过解释这种语言的句子来解决问题   
//!
//! 基本上，对于我们定义的任何类型的问题有如下三点：
//! - 领域专用语言
//! - 这种语言的语法
//! - 解决问题实例的解释器
//!
//! ### 描述
//!
//! ### 优点
//!
//! ### 缺点
//!
//! ### 讨论
//! Rust语言有macro_rules!支持定义特殊语法和如何展开这种语法为源代码的规则
//!
//! ### 执行
//! cargo r --example 3_1_2
//!

use crate::model::Interpreter;

mod model;

fn main() {
    let mut intr = Interpreter::new("2+3");
    let mut postfix = String::new();

    intr.interpret(&mut postfix);
    assert_eq!(postfix, "23+");

    postfix.clear();
    intr = Interpreter::new("1-2+3-4");
    intr.interpret(&mut postfix);
    assert_eq!(postfix, "12-3+4-");

    let x = -3f64;
    let y = 4f64;
    assert_eq!(3f64, norm!(x)); // √(-3)^2 = 3
    assert_eq!(5f64, norm!(x, y)); // √((-3)^2 + (4)^2) = 5
    assert_eq!(0f64, norm!(0, 0, 0)); // √((0)^2 + (0)^2 + (0)^2) = 0
    assert_eq!(1f64, norm!(0.5, -0.5, 0.5, -0.5)); // √((0.5)^2 + (-0.5)^2 + (0.5)^2 + (-0.5)^2) = 1
}
