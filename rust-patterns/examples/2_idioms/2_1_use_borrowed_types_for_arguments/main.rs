use crate::use_borrowed_types::{three_vowels, three_vowels2};

mod use_borrowed_types;

//! ## 以借用类型为参数
//!
//! ### 描述 Description
//! 使用借用类型(borrowed type)而不是自有数据类型的借用(borrowing the owned type)作为函数参数
//!
//! > 比如: &str而不是&String，&[T]而不是&Vec<T>，&T而不是&Box<T>
//!
//! ### 示例代码 Example
//!
//! ### 动机 Motivation
//!
//! ### 优点 Advantages
//!
//! ### 缺点 Disadvantages
//!
//! ### 讨论 Discussion
//!
//! ### 执行
//! cargo r --example 2_1
//!

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris)); //   Ferris: false
    println!("{}: {}", curious, three_vowels(&curious)); // Curious: true

    // 至此运行正常，但下面两行就会失败: &str不能强转为&String
    // println!("Ferris: {}", three_vowels("Ferris"));
    // println!("Curious: {}", three_vowels("Curious"));

    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels2(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }
    // curious has three consecutive vowels!
}
