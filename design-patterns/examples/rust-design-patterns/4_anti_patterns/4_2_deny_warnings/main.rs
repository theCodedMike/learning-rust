//! ## #![deny(warnings)]
//!
//! ### 描述
//! 一个善意的库作者想要确保他们的代码在编译时不会产生警告。因此他们在库里标注以下内容
//!
//! ### 优点
//! 它很短，如果有什么错误就停止构建
//!
//! ### 缺点
//! 通过禁用编译器生成警告，库的作者放弃了Rust的稳定性
//!
//! ### 替代方案
//! 1. 我们可以将编译设置与代码解耦
//! > RUSTFLAGS="-D warnings" cargo build
//! 2. 我们可以显式地命名要拒绝的警告
//! > #[deny(bad-style,
//! >        const-err,
//! >        dead-code,
//! >        improper-ctypes,
//! >        non-shorthand-field-patterns,
//! >        no-mangle-generic-items,
//! >        overflowing-literals,
//! >        path-statements ,
//! >        patterns-in-fns-without-body,
//! >        private-in-public,
//! >        unconditional-recursion,
//! >        unused,
//! >        unused-allocation,
//! >        unused-comparisons,
//! >        unused-parens,
//! >        while-true
//! > )]
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 4_2
//!

fn main() {
    #![deny(warnings)]

    // All is well.
}
