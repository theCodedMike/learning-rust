#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

/// 以下内容皆为补充
///
/// cargo r --bin atomic
///
/// ## 目录
/// ### Atomic 原子类型
///
/// ### 内存屏障
///
fn main() {
    /* Atomic 原子类型 */
    //atomic();

    /* 内存顺序 指CPU在访问内存时的顺序，该顺序可能受以下因素的影响:
       1: 代码中的先后顺序
       2: 编译器优化导致在编译阶段发生改变(内存重排序 reordering)
       3: 运行阶段因 CPU 的缓存机制导致顺序被打乱
    */
    memory_barrier();
}

///
/// Atomic types provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types.
///
/// Including AtomicBool,
///           AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize,
///           AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
///           AtomicPtr
///
/// Atomic memory orderings: Ordering   
///   - Ordering::Relaxed   No ordering constraints, only atomic operations.
///
///   - Ordering::Release   When coupled with a store, all previous operations become ordered before
///                         any load of this value with Acquire (or stronger) ordering.
///
///   - Ordering::Acquire   When coupled with a load, if the loaded value was written by a store
///                         operation with Release (or stronger) ordering, then all subsequent
///                         operations become ordered after that store.
///
///   - Ordering::AcqRel    Has the effects of both Acquire and Release together: For loads it uses
///                         Acquire ordering. For stores it uses the Release ordering.
///
///   - Ordering::SeqCst    Like Acquire/Release/AcqRel (for load, store, and load-with-store
///                         operations, respectively) with the additional guarantee that all threads
///                         see all sequentially consistent operations in the same order.
///
fn atomic() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = std::thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
        println!("child thread: {}", spinlock_clone.load(Ordering::SeqCst)); // 0
        spinlock_clone.fetch_add(10, Ordering::SeqCst);
        println!("child thread: {}", spinlock_clone.load(Ordering::SeqCst)); // 10
    });

    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::SeqCst) != 0 {
        std::hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!(" main Thread had an error: {panic:?}");
    }

    println!(" main thread: {}", spinlock.load(Ordering::SeqCst)); // 10
}

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);
fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}
fn producer() -> JoinHandle<()> {
    std::thread::spawn(|| {
        unsafe {
            DATA = 100; // A
        }
        READY.store(true, Ordering::Release); // B: 内存屏障
    })
}
fn consumer() -> JoinHandle<()> {
    std::thread::spawn(|| {
        while !READY.load(Ordering::Acquire) { // C: 内存屏障
        }
        assert_eq!(100, unsafe { DATA }); // D
    })
}
fn memory_barrier() {
    reset();
    let t_prod = producer();
    let t_cons = consumer();
    t_prod.join().unwrap();
    t_cons.join().unwrap();

    println!("{}", unsafe { DATA }); // 100
    println!("{}", READY.load(Ordering::Acquire)); // true
}
