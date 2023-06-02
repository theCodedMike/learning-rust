//! ## 扩展的秘密
//!
//! ### 描述
//! 被#[non_exhaustive]修饰的结构体或枚举，表明该结构体或枚举不完善，后续可能会添加成员。
//! 所以使用者需要注意兼容
//!
//! ### 优点
//!
//! ### 缺点
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_12
//!

use crate::privacy_for_extensibility::a::S;
use crate::privacy_for_extensibility::print_matched_variants;

mod privacy_for_extensibility;

fn main() {
    let s = S { foo: 5 };
    print_matched_variants(s);
}
