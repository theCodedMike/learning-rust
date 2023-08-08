#![allow(dead_code)]
#![allow(unused_variables)]

use crate::RefCellList::{Cons, Nil};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// 15.5 RefCell<T> 和内部可变性模式
///
/// cargo r --bin refcell
///
/// ## 目录
/// - 内部可变性（Interior mutability）是 Rust 中的一个设计模式
/// - 它允许你即使在有不可变引用时也可以改变数据
/// ### 通过 RefCell<T> 在运行时检查借用规则
/// - 在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的
/// - 对于 RefCell<T>，不可变性作用于 运行时而非编译时
/// - RefCell<T> / Cell<T> 只能用于单线程场景
///
/// 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由:
/// - Rc<T> 允许一份数据有多个所有者，而Box<T> 和 RefCell<T> 有单一所有者。
/// - Box<T> 允许在编译时执行不可变或可变借用检查，而Rc<T>仅允许在编译时执行不可变借用检查，RefCell<T> 允许在运行时执行不可变或可变借用检查。
/// - 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
///
/// ### 内部可变性: 不可变值的可变借用
/// #### 内部可变性的用例: mock 对象
///
/// ### RefCell<T> 在运行时记录借用
/// - RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针
/// - 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一
/// - RefCell<T> 在运行时只允许有多个不可变借用或一个可变借用，只是只在运行时检查这些引用规则
/// - 在运行时出现多个可变引用，程序会直接panic
///
/// ### 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
/// - 单线程环境: Rc<Cell<T>> Rc<RefCell<T>>
/// - 多线程环境: Arc<Mutex<T>>
///
fn main() {
    /* 通过 RefCell<T> 在运行时检查借用规则 */

    /* 内部可变性: 不可变值的可变借用 */
    let x = 5;
    //let y = &mut x; // cannot borrow `x` as mutable, as it is not declared as mutable
    // 内部可变性的用例: mock对象, 见 unit test

    /* RefCell<T> 在运行时记录借用 */
    let i32_4 = RefCell::new(4);
    //let copy_ref = i32_4.borrow(); // Ref<i32>, not &i32
    let copy_ref_mut = i32_4.borrow_mut(); // RefMut<i32>, not &mut i32
    let ref_cell = RefCell::new(String::from("hello"));
    //let borrow = ref_cell.borrow(); // Ref<String>, not &String
    let mut borrow_mut = ref_cell.borrow_mut(); // RefMut<String>, not &mut String
    borrow_mut.push_str(" rust");
    println!("{:?}", ref_cell); // RefCell { value: <borrowed> }
    println!("{}", borrow_mut); // hello rust
                                // let ref_2 = ref_cell.borrow(); // 编译时没有问题，运行时出错，already mutably borrowed: BorrowError
    println!();

    /* 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者 */
    // 3|b ------>
    //            |
    //           5|a -------> 10|d -------> Nil
    //            |
    // 4|c ------>
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a); // Cons(RefCell { value: 15 }, Nil)
    println!("b after = {:?}", b); // Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {:?}", c); // Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
    println!();

    /* 补充 */
    /* Cell<T> */
    let mut cell = Cell::new(4);
    // cell没有提供borrow相关的方法
    println!("{:?}", cell); // Cell { value: 4 }
    let get = cell.get();
    println!("get: {}, after get: {:?}", get, cell); // 4, Cell { value: 4 }
    let get_mut = cell.get_mut();
    *get_mut = 5;
    println!("after get_mut: {:?}", cell); // Cell { value: 5 }
    let take = cell.take();
    println!("take: {}, after take: {:?}", take, cell); // 4, Cell { value: 0 }
    cell.set(-10);
    println!("after set: {:?}", cell); // Cell { value: -10 }

    let mut cell = Cell::new(String::from("rust"));
    //println!("{}", cell); // 无法打印，提示Cell<String>未实现Debug
    let get_mut = cell.get_mut();
    println!("get_mut: {}", get_mut); // rust
    let get = &*get_mut;
    println!("get: {}", get); // rust, reborrow
    *get_mut = "OS".to_string();
    println!("get_mut: {}", get_mut); // OS
    let take = cell.take(); // take方法会将cell里的值set为T的默认值
    println!("take: {}", take); // OS
    cell.set("PingPang".to_string());
    print!("after set: {}", cell.into_inner()); // PingPang 注意into_inner方法会拿走cell的所有权

    // Cell VS. RefCell 方法对比
    //               Cell<T>       RefCell<T>
    // get()          √(T: Copy)       x
    // get_mut()      √                √
    // set()          √                x
    // take()         √                √
    // replace()      √                √
    // borrow()       x                √
    // borrow_mut()   x                √
    // ...
}

pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
#[derive(Debug)]
enum RefCellList<T> {
    Cons(Rc<RefCell<T>>, Rc<RefCellList<T>>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    /// 测试替身（test double）是一个通用编程概念，它代表一个在测试中替代某个类型的类型
    /// mock 对象 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的
    struct MockMessenger {
        //sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                //sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 这里的问题是: self是不可变引用，而push操作需要可变引用
            //self.sent_messages.push(String::from(message));

            // 这里有2个可变引用，编译时没问题，运行时检查借用规则，发现有问题
            // already borrowed: BorrowMutError
            //let ref_mut_one = self.sent_messages.borrow_mut();
            let mut ref_mut_two = self.sent_messages.borrow_mut();
            ref_mut_two.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        // here borrow
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
