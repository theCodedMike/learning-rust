//! ## 新类型
//!
//! ### 描述
//! 用带有单独字段的结构来创建一个类型的不透明包装器。这将创建一个新类型，而不是类型的别名
//!
//! 新类型的用处之一是可以绕开"孤儿规则"的限制
//!
//! ### 优点
//! 被包装的类型和包装后的类型是不兼容的，所以新类型的用户永远不会困惑于区分这二者的类型   
//! 新类型是零开销抽象——没有运行时负担   
//! 隐私系统确保用户不能访问包装的类型（如果字段是私有的，默认私有）
//!
//! ### 缺点
//! 有大量的啰嗦的样板代码
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_10
//!

use crate::model::Bar;

mod model;

fn main() {
    let _b = Bar::new();

    // Foo and Bar are type incompatible, the following do not type check.
    // let f: Foo = b;
    // let b: Bar = Foo { ... };
}
