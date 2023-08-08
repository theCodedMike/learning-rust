#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::sync::Arc;

/// 15.4 Rc<T> 引用计数智能指针
///
/// cargo r --bin rc
///
/// ## 目录
/// - 引用计数(reference counting): 意味着记录一个值引用的数量来知晓这个值是否仍在被使用, 即1份数据对应多个Owner
/// - Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候
/// - Rc<T> 只能用于单线程场景，如果想在多线程环境下使用，Rust提供了Arc<T> (Atomic Ref... Counting...)
/// - 只有 strong_count 为0时 Rc<T>/Arc<T> 实例才会被清理
/// ### 使用 Rc<T> 共享数据
/// 问题引入，具体见assets/multi_to_one.png，此时使用Box无法解决
///
/// ### 克隆 Rc<T> 会增加引用计数
/// - 使用 Rc<T> 允许一个值有多个所有者，且多个所有者只能共享数据
///
fn main() {
    // 3|b ------>
    //            |
    //           5|a -------> 10|d -------> Nil
    //            |
    // 4|c ------>

    /* 使用 Rc<T> 共享数据 */
    let a = List::new(5, List::new(10, List::Nil));
    let b = List::new(3, a);
    //let c = List::new(4, a); //编译报错，a的所有权已被转移到b中。所以Box无法解决共享数据问题

    let a = Rc::new(RcList::new(5, RcList::new(10, RcList::Nil)));
    let b = RcList::from(3, Rc::clone(&a)); // clone方法不是关联函数, a.clone()这种写法也可以
    let c = RcList::from(4, a); // a.clone()并不会克隆底层的数据，仅仅复制了一份指针

    /* 克隆 Rc<T> 会增加引用计数 */
    let a = Rc::new(RcList::new(5, RcList::new(10, RcList::Nil)));
    println!(
        "after creating a, strong count of a = {}",
        Rc::strong_count(&a)
    ); // 1
    let b = RcList::from(3, Rc::clone(&a));
    println!(
        "after creating b, strong count of a = {}",
        Rc::strong_count(&a)
    ); // 2
    {
        let c = RcList::from(4, Rc::clone(&a));
        println!(
            "after creating c, strong count of a = {}",
            Rc::strong_count(&a)
        ); // 3
    }
    println!(
        "after c goes out of scope, strong count of a = {}",
        Rc::strong_count(&a)
    ); // 2
    println!();

    /* 补充 */
    /* Rc无法更改内部数据(只实现了Deref，未实现DerefMut)，不像Box */
    let five = Rc::new(5);
    println!("{}", five); // 5
                          //*five += 1; // cannot assign to data in an `Rc`
    println!("{}", five); // 5
    println!();

    /* Arc: Atomic Rc, 同样只实现了Deref, 且实现了Send + Sync */
    let arc_s = Arc::new(String::from("多线程漫游者"));
    println!(
        "Arc: strong count of arc_s is {}",
        Arc::strong_count(&arc_s)
    );
    let mut handles = vec![];
    for i in 0..10 {
        let s = Arc::clone(&arc_s);
        let handle = std::thread::spawn(move || {
            println!(
                "Arc {}: {}, strong count of arc_s is {}",
                i,
                s,
                Arc::strong_count(&s)
            ) //打印10个 Arc: 多线程漫游者
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!(
        "Arc: strong count of arc_s is {}",
        Arc::strong_count(&arc_s)
    ); // 1 这里为1是因为clone增加的10个强引用在离开for循环后就自动减去了
}
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
impl<T> List<T> {
    fn new(val: T, next: List<T>) -> Self {
        List::Cons(val, Box::new(next))
    }
}
enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

impl<T> RcList<T> {
    fn new(val: T, next: RcList<T>) -> Self {
        //RcList::Cons(val, Rc::new(next))
        RcList::from(val, Rc::new(next))
    }
    fn from(val: T, next: Rc<RcList<T>>) -> Self {
        RcList::Cons(val, next)
    }
}
