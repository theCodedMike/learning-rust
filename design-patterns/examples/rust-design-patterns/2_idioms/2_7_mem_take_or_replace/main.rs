use crate::convert_enum::{a_to_b, swizzle, MultiVariateEnum, MyEnum};

mod convert_enum;

//! ## 在修改枚举时用mem::{take(_), replace(_)}保持值的所有权
//!
//! ### 描述
//! mem::take支持我们交换值，用默认值替换，并且返回原值   
//! mem:replace非常相似，不过其允许我们指定要替换的值   
//! mem::replace(name,String::new())
//!
//! ### 优点
//! 没有多余的内存申请，不用clone
//!
//! ### 缺点
//! take操作需要类型实现Default特性
//! 如果这个类型没有实现Default特性，你还是可以用 mem::replace
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_7
//!
fn main() {
    let mut a = MyEnum::A {
        name: "Mr. Hello".to_string(),
        x: 5,
    };
    println!("Before: {:?}", a);
    let res = a_to_b(&mut a);
    println!(" After: {:?}", a);
    println!("   {:?}", res);
    // A { name: "Mr. Hello", x: 5 }
    // A { name: "", x: 5 }
    // Some(B { name: "Mr. Hello" })

    println!("\n");

    let mut enum_a = MultiVariateEnum::A {
        name: "hello a".to_string(),
    };
    println!("Before: {:?}", enum_a);
    let res = swizzle(&mut enum_a);
    println!(" After: {:?}", enum_a);
    println!("    Now {:?}", res);
    // A { name: "hello a" }
    // A { name: "" }
    // B { name: "hello a" }
}
