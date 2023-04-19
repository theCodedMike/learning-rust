use serde::Serialize;

/// 15.1 使用Box<T>指向堆上的数据
///
/// cargo r --bin 15_1
///
/// ## 目录:
/// - box 允许你将一个值放在堆上而不是栈上，在栈上留有指向堆数据的指针
/// > 使用场景如下:
/// > - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
/// > - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
/// > - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
///
/// ### 使用 Box<T> 在堆上储存数据
/// ### Box 允许创建递归类型
/// - 递归类型（recursive type）: 其在编译时不知道大小
/// #### cons list 的更多内容
/// ### 计算非递归类型的大小
/// ### 使用 Box<T> 给递归类型一个已知的大小
///
fn main() {
    // 使用 Box<T> 在堆上储存数据
    let b = Box::new(5);
    println!("b = {}", b);
    let mut s = Box::new(String::from("hello"));
    s.push_str(" world");
    println!("s = {}", s);

    // Box 允许创建递归类型
    // cons list 的更多内容
    // 计算非递归类型的大小
    // 使用 Box<T> 给递归类型一个已知的大小
    let list = List::Cons(1,
                          Box::new(List::Cons(2,
                                              Box::new(List::Cons(3,
                                                                  Box::new(List::Nil))))));
    let string = serde_json::to_string_pretty(&list).unwrap();
    println!("{}", string);
}
// 递归类型，已解决
#[derive(Serialize)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}