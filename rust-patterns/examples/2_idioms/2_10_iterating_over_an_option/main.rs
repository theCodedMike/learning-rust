//! ## 关于Option的迭代器
//!
//! ### 描述
//! Option可以被视为一个包含一个0个或者1个元素的容器
//! Option实现了IntoIterator特性，它就可以用for循环来迭代
//!
//! ### 优点
//!
//! ### 缺点
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_10
//!
fn main() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];
    logicians.extend(turing);

    // equivalent to
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    let turing = Some("Turing");
    let logicians = vec!["Curry", "Kleene", "Markov"];

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }

    // Curry is a logician
    // Kleene is a logician
    // Markov is a logician
    // Turing is a logician
}
