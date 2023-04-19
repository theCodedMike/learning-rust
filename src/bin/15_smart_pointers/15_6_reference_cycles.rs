use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// 15.6 引用循环与内存泄漏
///
/// cargo r --bin 15_6
///
/// ## 目录:
/// ### 制造引用循环
/// ### 避免引用循环：将 Rc<T> 变为 Weak<T>
/// - 调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）
/// - 调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针
/// - 调用 Rc::downgrade 会将 weak_count 加1
/// - weak_count 无需计数为 0 就能使 Rc<T> 实例被清理
/// - 调用 Weak<T> 实例的 upgrade 方法会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None
/// #### 创建树形数据结构：带有子节点的 Node
/// #### 增加从子到父的引用
///
fn main() {
    //                   b
    //                   |
    //   a ---> 5|_ ---> 10|_ --->
    //          |                |
    //          <---------------<
    // 制造引用循环
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));  // 1
    println!("a next item = {:?}", a.tail()); // Nil

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // a

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());


    // 避免引用循环：将 Rc<T> 变为 Weak<T>

    // 创建树形数据结构：带有子节点的 Node
    // 增加从子到父的引用
    // 可视化 strong_count 和 weak_count 的改变
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("branch strong = {}, weak = {}",
                 Rc::strong_count(&branch),
                 Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}