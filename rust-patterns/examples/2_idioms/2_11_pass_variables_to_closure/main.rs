//! ## 向闭包传递变量
//!
//! ### 描述
//! 默认情况下，闭包从环境中借用捕获   
//! 你也可以用move闭包来将环境的所有权全给闭包   
//! 如果你是想传递一部分变量到闭包中，如一些数据的拷贝、传引用或者执行一些其他操作，
//! 这种情况应在不同的作用域里进行变量重绑定
//!
//! ### 优点
//!
//! ### 缺点
//! 增加了闭包内的实现代码行数
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_11
//!

use std::rc::Rc;

fn main() {
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);
    let closure = {
        // `num1` is moved
        let num2 = num2.clone(); // `num2` is cloned
        let num3 = num3.as_ref(); // `num3` is borrowed
        move || *num1 + *num2 + *num3
    };
    println!("{}", closure()); // 6

    // 不建议这么写！！！
    //
    // let num1 = Rc::new(1);
    // let num2 = Rc::new(2);
    // let num3 = Rc::new(3);
    // let num2_cloned = num2.clone();
    // let num3_borrowed = num3.as_ref();
    // let closure = move || {
    //     *num1 + *num2_cloned + *num3_borrowed;
    // };
}
