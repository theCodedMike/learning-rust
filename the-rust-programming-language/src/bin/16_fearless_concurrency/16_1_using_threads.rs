#![allow(dead_code)]

use std::cell::{Cell, OnceCell, RefCell};
use std::sync::{Arc, Barrier, Condvar, Mutex, Once, OnceLock};
use std::thread;
use std::thread::LocalKey;
use std::time::Duration;
use thread_local::ThreadLocal;

/// 16.1 在同一时间使用多线程
///
/// cargo r --bin thread
///
/// ## 目录
/// - 竞争状态(Race conditions): 多个线程以不一致的顺序访问数据或资源
/// - 死锁(Deadlocks): 两个线程相互等待对方停止使用其所拥有的资源
///
/// 线程模型:
/// - 1:1，一个 OS 线程对应一个语言级线程   Rust的选择，运行时开销小
/// - M:N，M个绿色线程对应 N 个 OS 线程
///
/// ### 使用 spawn 创建新线程
/// - 当主线程结束时，新线程也会结束，而不管其是否执行完毕
///
/// ### 使用 join 等待所有线程结束
/// - join 会阻塞当前线程直到 handle 所代表的线程结束
///
/// ### 线程与 move 闭包
///
/// ### 补充
/// #### 并发和并行
/// - 并发和并行都是对"多任务"处理的描述，其中并发是轮流处理，而并行是同时处理
/// #### CPU多核
/// - 单核心并发  多个线程维护多个任务队列，但只有1个CPU核心，所以多个队列只能轮流执行
/// - 多核心并行  当CPU核心增多到N时，那么同一时间就能有N个任务被处理
/// - 多核心并发  M个线程队列中，接着交给N个CPU核心去执行，最后实现M:N的处理模型，此时并发并行同时存在
/// #### 正式的定义
/// - 如果某个系统支持两个或者多个动作同时存在，那么这个系统就是一个并发系统
/// - 如果某个系统支持两个或者多个动作同时执行，那么这个系统就是一个并行系统
/// #### 编程语言的并发模型
/// - 由于操作系统提供了创建线程的 API，因此部分语言会直接调用该 API 来创建线程，因此最终程序内的线程数和该程序占用的操作系统线程数相等，一般称之为1:1 线程模型，例如 Rust
/// - 还有些语言在内部实现了自己的线程模型（绿色线程、协程），程序内部的 M 个线程最后会以某种映射方式使用 N 个操作系统线程去运行，因此称之为M:N 线程模型，例如 Go
/// - 还有些语言使用了 Actor 模型，基于消息传递进行并发，例如 Erlang
/// - Rust也支持M:N模型，见三方库tokio
///
fn main() {
    /* 使用 spawn 创建新线程 */
    /* 调用 join() 会阻塞主线程，等待所有子线程执行结束后主线程再执行 */
    //use_spawn_to_crate_a_thread();

    /* 线程与 move 闭包 */
    //use_move_to_pass_ownership();

    /* 补充 */
    /* 模拟子线程无法结束 */
    //simulate_endless();

    /* 线程屏障(Barrier)  可以让多个线程都执行到某个点后，才继续一起往后执行 */
    //thread_barrier();

    /* 线程局部变量(Thread Local Variable) 每个线程都有自己的局部变量 */
    //thread_local_std();
    //thread_local_std_another_style();
    //thread_local_3rd_crate();

    /* 用条件变量(Condition Variables)控制线程的挂起和执行 */
    //condition_var();
    //use_condition_var_1();
    //use_condition_var_2();

    /* 只能被调用一次的函数 Once */
    once();

    /* OnceCell / OnceLock */
    //once_cell();
    //once_lock();

    /* LazyCell / LazyLock unstable */
    lazy_cell();
    lazy_lock();
}
fn use_spawn_to_crate_a_thread() {
    /* 使用 spawn 创建新线程 */
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

    /* 调用 join() 会阻塞主线程，等待所有子线程执行结束后主线程再执行 */
    // 如果不调用join()，有可能主线程先执行完后整个程序就结束了，此时子线程可能还没执行完
    handle.join().unwrap();
}

fn use_move_to_pass_ownership() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    //println!("{:?}", v); // borrow of moved value: `v`
    handle.join().unwrap();
}

///
/// 主线程结束时子线程还没结束，此时程序直接结束
///
fn simulate_endless() {
    //创建子线程A
    let new_thread = thread::spawn(|| {
        //创建子线程B
        thread::spawn(|| loop {
            println!("I am a new thread B");
        })
    });
    new_thread.join().unwrap(); //等待新线程执行完成
    println!("child thread is finished");
    //睡眠一段时间，看子线程A创建的子线程B是否还在运行
    thread::sleep(Duration::from_millis(10));
    println!("main thread is finished");
}

///
/// 线程屏障
///
fn thread_barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for i in 0..6 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("thread {}: before wait", i);
            b.wait();
            println!("thread {}: after wait", i);
        }));
    }
    println!("now has {} strong_count", Arc::strong_count(&barrier)); // 7
    handles.into_iter().for_each(|h| h.join().unwrap());
    // thread 0: before wait
    // thread 1: before wait
    // thread 2: before wait
    // thread 4: before wait
    // thread 5: before wait
    // thread 3: before wait
    // thread 3: after wait
    // thread 4: after wait
    // thread 5: after wait
    // thread 1: after wait
    // thread 0: after wait
    // thread 2: after wait
}

thread_local! {
    static FOO: RefCell<u32> = RefCell::new(1)
}
///
/// thread local
///
fn thread_local_std() {
    FOO.with(|f| {
        println!("main thread: value is {}", f.borrow()); // 1
        *f.borrow_mut() = 3; //这里把main线程的thread_local改为3了
    });

    let handle = thread::spawn(move || {
        FOO.with(|f| {
            // 每个线程开始时都会拿到线程局部变量的FOO的初始值
            println!("child thread: value is {}", f.borrow()); // 1
            *f.borrow_mut() = 4; //这里把子线程的thread_local改为4了，不会影响main线程的值
            println!("child thread: value is {}", f.borrow()); // 4
        });
    });
    handle.join().unwrap();

    FOO.with(|f| {
        // 尽管子线程中修改为了4，我们在这里依然拥有main线程中的局部值：3
        println!("main thread: value is {}", *f.borrow()); // 3
    });
}

struct ThreadLocalAtStruct;
impl ThreadLocalAtStruct {
    thread_local! {
        static MY_AGE: RefCell<usize> = RefCell::new(18);
    }
}
thread_local! {
    static MY_MARK: RefCell<usize> = RefCell::new(50);
}
struct UseThreadLocalByRef {
    mark: &'static LocalKey<RefCell<usize>>,
}
impl UseThreadLocalByRef {
    fn new() -> Self {
        Self { mark: &MY_MARK }
    }
    fn get_mark(&self) -> usize {
        self.mark.with(|f| *f.borrow())
    }
    fn increase(&self, val: usize) -> usize {
        self.mark.with(|f| {
            *f.borrow_mut() += val;
            *f.borrow()
        })
    }
}
///
/// thread local at field
///
fn thread_local_std_another_style() {
    ThreadLocalAtStruct::MY_AGE.with(|f| {
        println!("use thread_local at struct: {}", f.borrow()); // 18
    });

    let thread_local_by_ref = UseThreadLocalByRef::new();
    println!(
        "before increase: mark is {}",
        thread_local_by_ref.get_mark()
    ); // 50
    thread_local_by_ref.increase(3);
    println!(
        " after increase: mark is {}",
        thread_local_by_ref.get_mark()
    ); // 53
}

///
/// 使用三方包 thread_local
///
/// 和标准库的thread local有点不同
///
fn thread_local_3rd_crate() {
    // Basic usage of ThreadLocal
    let tls: ThreadLocal<u32> = ThreadLocal::new();
    assert_eq!(tls.get(), None);
    assert_eq!(tls.get_or(|| 5), &5);
    assert_eq!(tls.get(), Some(&5));

    // Combining thread-local values into a single result
    let tls = Arc::new(ThreadLocal::new());
    // Create a bunch of threads to do stuff
    for i in 0..5 {
        let tls2 = tls.clone();
        thread::spawn(move || {
            // Increment a counter to count some event...
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
            println!("child thread {}: {}", i, cell.get());
        })
        .join()
        .unwrap(); // join()方法出现在这里，所以这5个线程是按顺序执行的
    }
    // Once all threads are done, collect the counter values and return the
    // sum of all thread-local counter values.
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| x + y.get());
    assert_eq!(total, 5);
}

///
/// 条件变量
///
fn condition_var() {
    //条件变量(Condition Variables)经常和 Mutex 一起使用，可以让线程挂起，直到某个条件发生后再继续执行
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move || {
        println!("child thread: 1");
        let (lock, cvar) = &*pair2;
        println!("child thread: 2");
        let mut started = lock.lock().unwrap();
        println!("child thread: 3, started: {}", started);
        *started = true;
        println!("child thread: 4, started: {}", started);
        // We notify the condvar that the value has changed.
        cvar.notify_one();
        println!("child thread: 5, started: {}", started);
    });
    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!(" main thread: 1, started: {}", started);
    while !*started {
        println!(" main thread: 2, started: {}", started);
        // Blocks the current thread until this condition variable receives a notification
        started = cvar.wait(started).unwrap();
        println!(" main thread: 3, started: {}", started);
    }
    println!(" main thread: 4, started: {}", started);

    //  main thread: 1, started: false
    //  main thread: 2, started: false
    // child thread: 1
    // child thread: 2
    // child thread: 3, started: false
    // child thread: 4, started: true
    // child thread: 5, started: true
    //  main thread: 3, started: true
    //  main thread: 4, started: true
}

#[derive(Clone, Debug, PartialEq)]
enum Next {
    Child,
    Main,
}
///
/// 使用条件变量示例1
///
/// 主、子线程交替打印  [1, 3]
///
fn use_condition_var_1() {
    let next = Arc::new(Mutex::new(Next::Main));
    let cond = Arc::new(Condvar::new());

    let next2 = next.clone();
    let cond2 = cond.clone();

    let handle = thread::spawn(move || {
        let lock = next2.lock().unwrap();
        let mut next_flag = (*lock).clone();
        drop(lock);
        for i in 1..=3 {
            while let Next::Main = next_flag {
                next_flag = (*cond2.wait(next2.lock().unwrap()).unwrap()).clone();
            } // next_flag 为 Next::Child 时跳出 while-loop
            eprintln!("child:\t{}", i);
            next_flag = Next::Main;
            *next2.lock().unwrap() = next_flag.clone(); // 下一个进行打印的是main线程
        }
    });
    for i in 1..=3 {
        eprintln!(" main:\t{}", i);
        let mut next_flag = next.lock().unwrap();
        *next_flag = Next::Child; // 下一个进行打印的是child线程
        drop(next_flag);
        cond.notify_one();
        thread::sleep(Duration::from_secs(1));
        //睡一秒, 给child线程提供上锁的机会.
    }
    handle.join().unwrap();

    //  main:  1
    // child:  1
    //  main:  2
    // child:  2
    //  main:  3
    // child:  3
}

///
/// 使用条件变量示例1
///
/// 主、子线程交替打印  [1, 10]
///
fn use_condition_var_2() {
    let arc = Arc::new((Mutex::new(Next::Main), Condvar::new()));
    let arc_clone = Arc::clone(&arc);
    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*arc_clone;
        let mut guard = lock.lock().unwrap();
        for i in 1..=10 {
            println!("child thread: {}", i);
            *guard = Next::Child;
            cvar.notify_one();
            while *guard == Next::Child {
                guard = cvar.wait(guard).unwrap();
            }
        }
    });
    let &(ref lock, ref cvar) = &*arc;
    let mut guard = lock.lock().unwrap();
    for j in 1..=10 {
        println!(" main thread: {}", j);
        while *guard == Next::Main {
            guard = cvar.wait(guard).unwrap();
        }
        *guard = Next::Main;
        cvar.notify_one();
    }
}

static mut VAL: usize = 0;
static INIT: Once = Once::new();
///
/// Once: 只会被调用一次
///
/// Once: A synchronization primitive which can be used to run a one-time global initialization.
///
/// Useful for one-time initialization for FFI or related functionality.
///
fn once() {
    // 无论是哪个线程先调用函数来初始化，都会保证全局变量只会被初始化一次，随后的其它线程调用就会被忽略
    let mut handles = vec![];
    handles.push(thread::spawn(move || {
        INIT.call_once(|| {
            println!("child thread 1");
            unsafe {
                VAL = 1;
            }
        });
    }));
    handles.push(thread::spawn(move || {
        INIT.call_once(|| unsafe {
            println!("child thread 2");
            VAL = 2;
        });
    }));

    handles.into_iter().for_each(|h| h.join().unwrap());
    // 1或2，取决于哪个线程先初始化完，如果是handle1，则输出1；否则输出2
    println!("Call once: {}", unsafe { VAL });
}

///
/// OnceCell: cell只能初始化一次，可以获取到内部值的可变引用，非线程安全
///
/// A cell which can be written to only once.
///
/// This allows obtaining a shared &T reference to its inner value without copying or replacing it (unlike Cell),
/// and without runtime borrow checks (unlike RefCell).
///
/// For a thread-safe version of this struct, see std::sync::OnceLock.
///
fn once_cell() {
    let mut cell = OnceCell::new();
    assert!(cell.get().is_none());

    cell.get_or_init(|| "Hello, World!".to_string());
    println!("{:?}", cell.get()); // Some("Hello, World!")

    cell.get_or_init(|| "Hello Java".to_string()); // 这次的init不起作用
    println!("{:?}", cell.get()); // Some("Hello, World!")

    cell.get_mut().map(|cell| *cell = "Hello Rust".to_string());
    println!("{:?}", cell.get()); // Some("Hello Rust")
}

///
/// OnceLock: OnceCell的线程安全版本
///
fn once_lock() {
    unsafe {
        static mut CELL: OnceLock<String> = OnceLock::new();
        assert!(CELL.get().is_none());

        thread::spawn(|| {
            let value: &String = CELL.get_or_init(|| "Hello, World!".to_string());
            println!("child thread: {}", value); // Hello, World!

            CELL.get_or_init(|| "Hello C++".to_string());
            println!("child thread: {:?}", CELL.get()); // Hello, World!

            CELL.get_mut().map(|cell| *cell = "Hello Rust".to_string());
            println!("child thread: {:?}", CELL.get()); // "Hello Rust"
        })
        .join()
        .unwrap();

        println!(" main thread: {:?}", CELL.get()); // "Hello Rust"
    }
}

///
/// LazyCell: A value which is initialized on the first access.
///
/// Non thread-safe
///
fn lazy_cell() {
    // unstable
}

///
/// LazyLock: A value which is initialized on the first access.
///
/// This type is a thread-safe LazyCell, and can be used in statics.
///
fn lazy_lock() {
    // unstable
}
