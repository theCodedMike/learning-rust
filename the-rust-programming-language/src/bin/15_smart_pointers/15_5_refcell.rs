#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// 15.5 RefCell<T> 和内部可变性模式
///
/// cargo r --bin refcell
///
/// ## 目录:
/// - 内部可变性（Interior mutability）是 Rust 中的一个设计模式
/// - 它允许你即使在有不可变引用时也可以改变数据
/// ### 通过 RefCell<T> 在运行时检查借用规则
/// - 对于 RefCell<T>，不可变性作用于 运行时而非编译时
/// - RefCell<T> 只能用于单线程场景
///
/// 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
/// - Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
/// - Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
/// - 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
/// ### 内部可变性：不可变值的可变借用
/// ### 内部可变性的用例：mock 对象
/// ### RefCell<T> 在运行时记录借用
/// - RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针
/// - 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一
/// - RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用
/// - 在运行时出现多个可变引用，程序会直接panic
///
/// ### 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
/// - Cell<T>: 作为实现了Copy特征的不可变类型的内部可变性
/// - Mutex<T>: 在多线程环境下提供内部可变性
///
fn main() {
    // 通过 RefCell<T> 在运行时检查借用规则
    // 内部可变性：不可变值的可变借用
    let x = 5;
    //let y = &mut x;

    // 内部可变性的用例：mock 对象
    // RefCell<T> 在运行时记录借用
    let ref_cell = RefCell::new(4);
    let ref_cell_str = RefCell::new(String::from("hello"));
    println!("{:?}", ref_cell); // RefCell { value: 4 }
    println!("{:?}", ref_cell_str); // RefCell { value: "hello" }

    // 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List3::Cons(Rc::clone(&value), Rc::new(List3::Nil)));
    let b = List3::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = List3::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let cell = Cell::new(4);
    let cell_str = Cell::new(String::from("hello"));
    // cell没有提供borrow相关的方法
    println!("{:?}", cell); // Cell { value: 4 }
    cell_str.set("world".to_string());
    println!("{:?}", cell_str.take()); // "world"
                                       //println!("now cell: {:?}", cell_str); //无法打印
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
enum List3 {
    Cons(Rc<RefCell<i32>>, Rc<List3>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    /// 测试替身（test double）是一个通用编程概念，它代表一个在测试中替代某个类型的类型
    /// mock 对象 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // here borrow_mut
            self.sent_messages.borrow_mut().push(String::from(message));
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
