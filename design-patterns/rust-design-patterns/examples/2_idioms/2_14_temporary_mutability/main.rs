//! ## 临时可变性
//!
//! ### 描述
//!
//! ### 优点
//! 编译器可以确保你之后不会意外修改数据
//!
//! ### 缺点
//! 多增加了一些本不必要的代码，代码结构更复杂
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_14
//!

fn main() {
    // 用代码块:
    let data = {
        let mut data = get_vec();
        data.sort();
        data
    };
    // Here `data` is immutable.
    println!("{:?}", data); // [1, 2, 2, 4, 5, 9, 9]

    // 用变量重绑定:
    let mut data = get_vec();
    data.sort();
    let data = data;
    // Here `data` is immutable.
    println!("{:?}", data); // [1, 2, 2, 4, 5, 9, 9]
}

fn get_vec() -> Vec<u8> {
    vec![5, 2, 4, 2, 9, 1, 9]
}
