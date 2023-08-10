/// 16.4 使用 Sync 和 Send 特征的可扩展并发
///
/// cargo r --bin sync-and-send
///
/// ## 目录
/// ### 通过 Send 允许在线程间转移所有权
/// - Send trait 表明实现了 Send 的类型值的所有权可以在线程间传递
/// - 几乎所有的 Rust 类型都实现了 Send，除了 Rc<T>、裸指针(raw pointer)等
/// - 完全由 Send 的类型组成的类型也会自动被标记为 Send
///
/// ### Sync 允许多线程访问
/// - Sync trait 表明实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用
/// - 对于任意类型 T，如果 &T是 Send 的话 T 就是 Sync 的
/// - 完全由 Sync 的类型组成的类型也是 Sync 的
/// - 未实现Sync的常见类型有Rc<T>、Cell<T>、RefCell<T>
///
/// ### 手动实现 Send 和 Sync 是不安全的
/// - 通常并不需要手动实现 Send 和 Sync trait
///
fn main() {
    /* 通过 Send 允许在线程间转移所有权 */

    /* Sync 允许多线程访问 */

    /* 手动实现 Send 和 Sync 是不安全的 */

    /* 补充 */
    // 为裸指针实现Send + Sync
    impl_send_and_sync();
}

#[derive(Debug)]
struct RawPointerBox(*const u8);
unsafe impl Send for RawPointerBox {}
unsafe impl Sync for RawPointerBox {}

#[derive(Debug)]
struct NotImplRawPointer(*const u8);

///
/// 为裸指针实现Send + Sync
///
/// 这个例子是为了展示，不建议手动实现这2个Trait
///
/// 这种实现方式就是newtype，即使用元组结构体绕开"孤儿规则"
///
fn impl_send_and_sync() {
    let pointer = 5 as *mut u8;

    let pointer_box = RawPointerBox(pointer);
    //let pointer_box = NotImplRawPointer(pointer); // `*const u8` cannot be sent between threads safely

    std::thread::spawn(move || {
        println!("raw pointer: {:?}", pointer_box); //RawPointerBox(0x5)
    })
    .join()
    .unwrap();

    println!("main thread end...");
}
