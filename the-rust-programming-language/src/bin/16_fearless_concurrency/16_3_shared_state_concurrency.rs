#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use tokio::sync::{Semaphore, TryAcquireError};

/// 16.3 共享状态并发
///
/// cargo r --bin mutex
///
/// ## 目录
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
/// #### 在线程间共享 Mutex<T>
/// #### 多线程和多所有权
/// #### 原子引用计数 Arc<T>
/// - 原子引用计数（atomically reference counted）
///
/// ### RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
/// - Mutex<T> 有造成 死锁（deadlock） 的风险
///
#[tokio::main]
async fn main() {
    /* 互斥器: 一次只允许一个线程访问数据 */

    /* Mutex<T>的 API */
    //use_mutex();
    // 在线程间共享 Mutex<T>
    // 多线程和多所有权
    // 原子引用计数 Arc<T>
    //share_mutex_between_threads();

    /* RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性 */

    /* 补充 */
    /* 注意释放锁 */
    //use_multi_lock_in_single_thread();
    // 死锁 在另一个锁还未被释放时去申请新的锁就会触发
    //deadlock_in_single_thread();
    //deadlock_in_multi_threads();
    //use_try_lock_solve_deadlock();

    /* 读写锁 RwLock */
    //rwlock();

    /* 三方库提供的锁实现  parking_lot, spin */
    // parking_lot provides implementations of Mutex, RwLock, Condvar and Once that are smaller,
    // faster and more flexible than those in the Rust standard library

    /* 信号量 Semaphore */
    //semaphore().await;
    //semaphore_advance().await;
}

///
/// 基本使用
///
fn use_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m); // Mutex { data: 6, poisoned: false, .. }
    println!();
}

///
/// 在多线程环境中使用互斥锁Mutex
///
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

    println!("Result: {}", *counter.lock().unwrap()); // 10
}

///
/// 在单线程中多次使用Mutex
///
fn use_multi_lock_in_single_thread() {
    let m = Mutex::new(5);

    let mut num = m.lock().unwrap();
    *num = 6;
    // 锁还没有被 drop 就尝试申请下一个锁，会导致主线程阻塞
    drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    drop(num1); // 手动 drop num1 ，观察打印结果的不同

    //println!("m = {:?}", m); // 如果没有drop(num1)  Mutex { data: <locked>, poisoned: false, .. }
    println!("m = {:?}", m); // 如果drop(num1)     Mutex { data: 7, poisoned: false, .. }
}

///
/// 单线程中复现死锁
///
/// 解锁方式1: 使用代码块, 离开代码块时自动释放锁  
/// 解锁方式2: 手动drop
///
fn deadlock_in_single_thread() {
    //在另一个锁还未被释放时去申请新的锁就会触发
    let data = Mutex::new(0);
    {
        let d1 = data.lock();
    }
    //drop(d1);
    {
        let d2 = data.lock();
    }
    //drop(d2);
}

static MUTEX1: Mutex<i64> = Mutex::new(0);
static MUTEX2: Mutex<i64> = Mutex::new(1);
///
/// 多线程中复现死锁
///
fn deadlock_in_multi_threads() {
    let mut children = vec![]; // 存放子线程的句柄
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            // 线程1
            if i_thread % 2 == 0 {
                let guard = MUTEX1.lock().unwrap();
                println!("线程 {} 锁住了MUTEX1，接着去锁住MUTEX2 ！", i_thread);
                drop(guard); // 如果注释了drop，就可能会发生死锁
                             //当前线程睡眠一小会，等待线程2去锁住MUTEX2
                thread::sleep(Duration::from_millis(10));
                //去锁住MUTEX2
                let guard = MUTEX2.lock().unwrap();
                println!("线程 {} 获取MUTEX2锁的结果：{:?}", i_thread, guard);
            } else {
                //锁住MUTEX2
                let _guard = MUTEX2.lock().unwrap();
                println!("线程 {} 锁住了MUTEX2，接着去锁住MUTEX1 ！", i_thread);
                drop(_guard); // 如果注释了drop，就可能会发生死锁
                thread::sleep(Duration::from_millis(10));
                let _guard = MUTEX1.lock().unwrap();
                println!("线程 {} 获取MUTEX1锁的结果：{:?}", i_thread, _guard);
            }
        }));
    }
    for child in children {
        // 等子线程完成
        let result = child.join();
    }

    // 线程 0 锁住了MUTEX1，接着去锁住MUTEX2 ！
    // 线程 1 锁住了MUTEX2，接着去锁住MUTEX1 ！
    // 线程 1 获取MUTEX1锁的结果：0
    // 线程 0 获取MUTEX2锁的结果：1
}

///
/// 使用try_lock()
///
fn use_try_lock_solve_deadlock() {
    let mut children = vec![]; // 存放子线程的句柄
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            // 线程1
            if i_thread % 2 == 0 {
                let guard = MUTEX1.lock().unwrap();
                println!("线程 {} 锁住了MUTEX1，接着去锁住MUTEX2 ！", i_thread);
                //当前线程睡眠一小会，等待线程2去锁住MUTEX2
                thread::sleep(Duration::from_millis(10));
                //去锁住MUTEX2
                let guard = MUTEX2.try_lock();
                println!("线程 {} 获取MUTEX2锁的结果：{:?}", i_thread, guard);
            } else {
                //锁住MUTEX2
                let _guard = MUTEX2.lock().unwrap();
                println!("线程 {} 锁住了MUTEX2，接着去锁住MUTEX1 ！", i_thread);
                thread::sleep(Duration::from_millis(10));
                let _guard = MUTEX1.try_lock();
                println!("线程 {} 获取MUTEX1锁的结果：{:?}", i_thread, _guard);
            }
        }));
    }
    for child in children {
        // 等子线程完成
        let result = child.join();
    }
    println!("deadlock没有发生");

    // 线程 0 锁住了MUTEX1，接着去锁住MUTEX2 ！
    // 线程 1 锁住了MUTEX2，接着去锁住MUTEX1 ！
    // 线程 0 获取MUTEX2锁的结果：Err("WouldBlock")
    // 线程 1 获取MUTEX1锁的结果：Err("WouldBlock")
    // deadlock没有发生
}

/// A reader-writer lock
///
/// This type of lock allows a number of readers or at most one writer at any point in time.
///
/// a Mutex does not distinguish between readers or writers that acquire the lock, therefore blocking any threads waiting for the lock to become available.
///
/// An RwLock will allow any number of readers to acquire the lock as long as a writer is not holding the lock.
///
fn rwlock() {
    let lock = RwLock::new(String::from("rwlock"));
    // many reader locks can be held at once
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("r1: {}", r1); // rwlock
        println!("r2: {}", r2); // rwlock
    } // read locks are dropped at this point

    // only one write lock may be held, however
    {
        let mut w1 = lock.write().unwrap();
        *w1 += " is shit";
        println!("{}", w1); // rwlock is shit
        drop(w1); // 如果这里不手动drop，则会形成死锁
        let mut w2 = lock.write().unwrap();
        *w2 += ", and is great";
        println!("{}", w2); // rwlock is shit, and is great
    } // write lock is dropped here
}

///
/// 信号量不是由标准库提供的，而是由 tokio 提供
///
/// A semaphore maintains a set of permits. Permits are used to synchronize access to a shared resource.
///
/// A semaphore differs from a mutex in that it can allow more than one concurrent caller to access the shared resource at a time.
///
async fn semaphore() {
    // basic usage
    let semaphore = Semaphore::new(3);
    let a_permit = semaphore.acquire().await.unwrap();
    println!("{:?}", a_permit); // SemaphorePermit { sem: Semaphore { ll_sem: Semaphore { permits: 2 } }, permits: 1 }
    let two_permits = semaphore.acquire_many(2).await.unwrap();
    println!("{:?}", two_permits); // SemaphorePermit { sem: Semaphore { ll_sem: Semaphore { permits: 0 } }, permits: 2 }
    assert_eq!(semaphore.available_permits(), 0);

    let permit_attempt = semaphore.try_acquire();
    assert_eq!(permit_attempt.err(), Some(TryAcquireError::NoPermits));
}

///
/// Use Semaphore::acquire_owned to move permits across tasks
///
async fn semaphore_advance() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            // perform task...
            // explicitly own `permit` in the task
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
