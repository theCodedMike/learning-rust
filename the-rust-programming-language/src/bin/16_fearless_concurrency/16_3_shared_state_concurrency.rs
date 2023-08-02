#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::{Arc, Mutex};
use std::thread;

/// 16.3 共享状态并发
///
/// cargo r --bin concurrency
///
/// ## 目录:
/// - 共享内存类似于多所有权：多个线程可以同时访问相同的内存位置
///
/// ### 互斥器一次只允许一个线程访问数据
/// - mutex: mutual exclusion
/// - 任意时刻，其只允许一个线程访问某些数据
/// - 使用互斥锁的3个步骤:
/// > 1、获取锁
/// > 2、处理数据
/// > 3、释放锁
///
/// ### Mutex<T>的 API
/// ### 在线程间共享 Mutex<T>
/// ### 多线程和多所有权
/// ### 原子引用计数 Arc<T>
/// - 原子引用计数（atomically reference counted）
/// ### RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
/// - Mutex<T> 有造成 死锁（deadlock） 的风险
fn main() {
    // 互斥器一次只允许一个线程访问数据
    // Mutex<T>的 API
    //use_mutex();

    // 在线程间共享 Mutex<T>
    // 多线程和多所有权
    // 原子引用计数 Arc<T>
    share_mutex_between_threads();

    // RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
}
fn use_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m); // Mutex { data: 6, poisoned: false, .. }
}
fn share_mutex_between_threads() {
    // 这里不能用Rc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
