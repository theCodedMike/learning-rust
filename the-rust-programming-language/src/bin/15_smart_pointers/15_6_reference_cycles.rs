#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// 15.6 引用循环与内存泄漏
///
/// cargo r --bin ref-cycle
///
/// ## 目录:
/// ### 制造引用循环
///
/// ### 避免引用循环：将 Rc<T> 变为 Weak<T>
/// - 调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）
/// - 调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针
/// - 调用 Rc::downgrade 会将 weak_count 加1
/// - weak_count 无需计数为 0, Rc<T> 实例就会被清理(strong_count为0时实例才会被清理)
/// - 调用 Weak<T> 实例的 upgrade 方法会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None
/// #### 创建树形数据结构：带有子节点的 Node
/// #### 增加从子到父的引用
/// #### 可视化 strong_count 和 weak_count 的改变
///
fn main() {
    //
    //   5|a ---> 10|b
    //    ↑         ↓
    //     ---------
    //
    /* 制造引用循环 */
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail()); // Nil

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // a: Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());
    println!();

    /* 避免引用循环：将 Rc<T> 变为 Weak<T> */
    // 创建树形数据结构: 带有子节点的 Node
    //
    //        5|branch
    //     /       \
    //   3|leaf   10|leaf

    let leaf_10 = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let leaf_3 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf_3 strong = {}, weak = {}, parent = {:?}",
        Rc::strong_count(&leaf_3),
        Rc::weak_count(&leaf_3),
        leaf_3.parent.borrow().upgrade()
    ); // strong = 1, weak = 0, parent = None

    {
        let branch_5 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf_3), Rc::clone(&leaf_10)]),
        });
        // 增加从子到父的引用
        *leaf_3.parent.borrow_mut() = Rc::downgrade(&branch_5);
        *leaf_10.parent.borrow_mut() = Rc::downgrade(&branch_5);
        println!(
            "branch_5 strong = {}, weak = {}",
            Rc::strong_count(&branch_5),
            Rc::weak_count(&branch_5),
        ); // strong = 1, weak = 2, parent =
           // Some(Node {
           //             value: 5,
           //             parent: RefCell { value: (Weak) },
           //             children: RefCell { value: [
           //                                            Node {
           //                                                   value: 3,
           //                                                   parent: RefCell { value: (Weak) },
           //                                                   children: RefCell { value: [] }
           //                                            },
           //                                            Node {
           //                                                   value: 10,
           //                                                   parent: RefCell { value: (Weak) },
           //                                                   children: RefCell { value: [] }
           //                                            }
           //                                        ]
           //                               }
           //           }
           // )

        println!(
            "leaf_3 strong = {}, weak = {}, parent = {:?}",
            Rc::strong_count(&leaf_3),
            Rc::weak_count(&leaf_3),
            leaf_3.parent.borrow().upgrade()
        ); // strong = 2, weak = 0
    }

    // 可视化 strong_count 和 weak_count 的改变
    println!(
        "leaf_3 strong = {}, weak = {}, parent = {:?}",
        Rc::strong_count(&leaf_3),
        Rc::weak_count(&leaf_3),
        leaf_3.parent.borrow().upgrade()
    ); // strong = 1, weak = 0

    /* 补充 */
    /* Rc<T> Vs. Weak<T> */
    //       Weak                                               Rc
    // 一般通过Rc::downgrade()方法来创建实例              一般通过Rc::new()/Rc::clone()来创建实例
    // 不拥有数据的所有权                                 拥有数据的所有权
    // 与数据被释放没有关系                                强引用计数为0时实例才能被释放
    // 通过upgrade()方法取得Option<Rc<T>>, 然后再操作值     由于Deref, 可以直接操作值
    let five = Rc::new(5);
    println!(
        "strong: {}, weak: {}",
        Rc::strong_count(&five),
        Rc::weak_count(&five)
    ); // 1 0
    let weak = Rc::downgrade(&five);
    println!(
        "after downgrade: strong: {}, weak: {}",
        Rc::strong_count(&five),
        Rc::weak_count(&five)
    ); // 1 1
    println!(
        "after downgrade: strong: {}, weak: {}",
        weak.strong_count(),
        weak.weak_count()
    ); // 1 1
    let option = weak.upgrade();
    println!("{:?}", option); // Some(5)
    if let Some(rc) = option {
        println!(
            "after upgrade: strong: {}, weak: {}",
            Rc::strong_count(&rc),
            Rc::weak_count(&rc)
        ); // 2 1
        println!(
            "after upgrade: strong: {}, weak: {}",
            Rc::strong_count(&five),
            Rc::weak_count(&five)
        ); // 2 1
        drop(rc);
        println!(
            "after drop rc: strong: {}, weak: {}",
            Rc::strong_count(&five),
            Rc::weak_count(&five)
        ); // 1 1
    }
    drop(five);
    println!(
        "after drop val: strong: {}, weak: {}",
        weak.strong_count(),
        weak.weak_count()
    ); // 0 0
    let option1 = weak.upgrade();
    println!("after drop val: {:?}", option1); // None
    println!();

    // 一个例子  每个家具都有其主人，而多个家具只能拥有一个主人
    let owner = Rc::new(Owner::new("Smith"));
    let gadget1 = Rc::new(Gadget::new(1, owner.clone()));
    let gadget2 = Rc::new(Gadget::new(2, owner.clone()));
    owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
    owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));
    println!(
        "owner: s_count: {}, w_count: {}",
        Rc::strong_count(&owner),
        Rc::weak_count(&owner)
    ); // 3 0
    println!(
        "gadget1: s_count: {}, w_count: {}",
        Rc::strong_count(&gadget1),
        Rc::weak_count(&gadget1)
    ); // 1 1
    println!(
        "gadget2: s_count: {}, w_count: {}",
        Rc::strong_count(&gadget2),
        Rc::weak_count(&gadget2)
    ); // 1 1
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

// 主人
struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

impl Owner {
    fn new(name: &str) -> Self {
        Owner {
            name: name.to_string(),
            gadgets: RefCell::new(Vec::new()),
        }
    }
}

// 家具
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

impl Gadget {
    fn new(id: i32, owner: Rc<Owner>) -> Self {
        Gadget { id, owner }
    }
}
