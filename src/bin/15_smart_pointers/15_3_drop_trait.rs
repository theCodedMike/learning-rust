/// 15.3 使用 Drop Trait 运行清理代码
///
/// cargo r --bin 15_3
///
/// ## 目录:
/// - Drop trait允许在值要离开作用域时执行一些代码
/// - 变量以被创建时相反的顺序被丢弃
/// ### 通过 std::mem::drop 提早丢弃值
/// - Rust 并不允许我们主动调用 Drop trait 的 drop 方法
/// - 当我们希望在作用域结束之前就强制释放变量时，可以使用由标准库提供的 std::mem::drop
///
fn main() {
    //let _c = CustomSmartPointer { data: String::from("my stuff") };    // 3
    //let _d = CustomSmartPointer { data: String::from("other stuff") }; // 2
    //println!("CustomSmartPointers created."); // 1

    // 通过 std::mem::drop 提早丢弃值
    let c = CustomSmartPointer {
        data: String::from("some data"),
    }; // 2
    println!("CustomSmartPointer created."); // 1
                                             //c.drop(); // explicit destructor calls not allowed
    drop(c);
    println!("CustomSmartPointer dropped before the end of main."); // 3
}
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
