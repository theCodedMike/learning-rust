//! ## 确定性析构
//!
//! ### 描述
//! Rust不提供与finally等价的代码块(即不管函数怎么结束都会执行的代码)   
//! 一个对象的析构器将会执行在退出前必须执行的代码
//!
//! ### 优点
//! 在析构器里的代码退出前总是会被执行，能应对恐慌（panics），提前返回等等
//!
//! ### 缺点
//! 不保证析构器里的代码一定会被执行   
//! 举例来说，函数内有一个死循环或者在退出前函数崩溃的情况   
//! 为了确定性，申请一个对象和实现Drop特性增加了很多样板代码
//!
//! ### 讨论
//! 在Rust中，析构器在对象离开作用域的时候执行   
//! 当恐慌发生时， Rust对每个栈帧中的每个对象执行析构器代码   
//! 如果一个析构器在析构时出现了恐慌，则Rust不再执行析构，而会果断终止这个线程   
//! 这就意味着Rust并不是绝对保证析构器一定会执行，因此可能会导致资源泄露
//!
//! ### 执行
//! cargo r --example 2_6
//!
fn main() {
    if bar().is_err() {
        println!("Error occurred");
    }
    // exit
    // Error occurred
}

fn bar() -> Result<(), ()> {
    // These don't need to be defined inside the function.
    #[derive(Debug)]
    struct Foo;

    // Implement a destructor for Foo.
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    // The dtor of _exit will run however the function `bar` is exited.
    let exit = Foo;

    // Implicit return with `?` operator.
    baz()?;

    println!("{:?}", exit);

    // Normal return.
    Ok(())
}
fn baz() -> Result<(), ()> {
    Err(())
}
