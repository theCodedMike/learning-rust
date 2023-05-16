use std::thread;
use std::time::Duration;

/// 16.1 在同一时间使用多线程
///
/// cargo r --bin 16_1
///
/// ## 目录:
/// - 竞争状态（Race conditions）: 多个线程以不一致的顺序访问数据或资源
/// - 死锁（Deadlocks）: 两个线程相互等待对方停止使用其所拥有的资源
///
/// 线程模型:
/// - 1:1，一个 OS 线程对应一个语言线程   Rust的选择，运行时开销小
/// - M:N，M个绿色线程对应 N 个 OS 线程
///
/// ### 使用 spawn 创建新线程
/// - 当主线程结束时，新线程也会结束，而不管其是否执行完毕
/// ### 使用 join 等待所有线程结束
/// - join 会阻塞当前线程直到 handle 所代表的线程结束
/// ### 线程与 move 闭包
///
fn main() {
    // 使用 spawn 创建新线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread: hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap(); //如果在这里join，则子线程先执行，执行完以后主线程才执行
    for i in 1..5 {
        println!("   main thread: hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 使用 join 等待所有线程结束
    handle.join().unwrap();

    // 线程与 move 闭包
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
