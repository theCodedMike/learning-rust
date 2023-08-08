#![allow(unused)]
#![allow(dropping_copy_types)]

/// 15.3 使用 Drop Trait 运行清理代码
///
/// cargo r --bin drop
///
/// ## 目录
/// - Drop trait允许在值要离开作用域时执行一些代码
/// - 变量以被创建时相反的顺序被丢弃
///
/// ### 通过 std::mem::drop 提早丢弃值
/// - Rust 并不允许我们主动调用 Drop trait 的 drop 方法
/// - 当我们希望在作用域结束之前就强制释放变量时，可以使用由标准库提供的 std::mem::drop
///
fn main() {
    /* 使用 Drop Trait 运行清理代码 */
    // 当实例离开作用域 Rust 会自动调用 drop，并调用我们指定的代码，无需显式调用drop方法
    let c = CustomSmartPointer::new("my stuff"); // 3
    let d = CustomSmartPointer::new("other stuff"); // 2
    println!("CustomSmartPointers created."); // 1
    println!();

    /* 通过 std::mem::drop 提早丢弃值 */
    let c = CustomSmartPointer::new("some data"); // 2
    println!("CustomSmartPointer created."); // 1
                                             //c.drop(); // explicit destructor(析构函数) calls not allowed
    std::mem::drop(c); // 这里会拿走c的所有权
    println!("CustomSmartPointer dropped before the end of main."); // 3
    println!();

    /* 补充 */
    /* drop的顺序 */
    // 变量级别，按照逆序的方式
    // 结构体内部，按照顺序的方式

    /* Copy和Drop 互斥 不能为一个类型同时实现 Copy 和 Drop Trait*/
    let i = 5;
    std::mem::drop(i); // i实现了Copy，此时drop方法失去了作用
    println!("{}", i); // 5
}
struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(data: &str) -> Self {
        CustomSmartPointer {
            data: data.to_string(),
        }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
