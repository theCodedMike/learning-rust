#![allow(dead_code)]
#![allow(unused_variables)]

use serde::Serialize;

/// 15.1 使用Box<T>指向堆上的数据
///
/// cargo r --bin box
///
/// ## 目录
/// box 允许你将一个值放在堆上而不是栈上，在栈上留有指向堆数据的指针
///
/// 使用场景如下:
/// - 特意将数据分配在堆上
/// - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
/// - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
/// - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
///
/// > 栈内存从高位地址向下增长，且栈内存是连续分配的，一般来说操作系统对栈内存的大小都有限制  
/// > 在 Rust 中，main 线程的栈大小是 8MB，普通线程是 2MB  
/// > 堆上内存则是从低位地址向上增长，堆内存通常只受物理内存限制，且通常是不连续的  
///
///
/// ### 使用 Box<T> 在堆上储存数据
///
/// ### Box 允许创建递归类型
/// - 递归类型（recursive type）: 其在编译时不知道大小
/// #### cons list 的更多内容
///
/// ### 计算非递归类型的大小
///
/// ### 使用 Box<T> 给递归类型一个已知的大小
///
fn main() {
    /* 使用 Box<T> 在堆上储存数据 */
    let a = 5;
    println!("before box: {} bytes", std::mem::size_of_val(&a)); // 4 bytes
    let mut boxed_a = Box::new(a);
    println!(" after box: {} bytes", std::mem::size_of_val(&boxed_a)); // 8 bytes
    println!("b = {}", boxed_a); // 5
                                 //let sum = boxed_a + 1; // cannot add `{integer}` to `Box<{integer}>`
    *boxed_a += 1;
    println!("b = {}", boxed_a); // 6

    let b = String::from("hello");
    println!(
        "before box: {} bytes, len: {}, cap: {}",
        std::mem::size_of_val(&b),
        b.len(),
        b.capacity()
    ); // 24 bytes len:5 cap:5
    let mut boxed_b = Box::new(b);
    println!(" after box: {} bytes", std::mem::size_of_val(&boxed_b)); // 8 bytes
    boxed_b.push_str(" rust"); // 这里会自动解引用 &mut Box<String> -> &mut String
    println!(" after box: {} bytes", std::mem::size_of_val(&boxed_b)); // 8 bytes
    println!("s = {}", boxed_b); // hello rust
    println!();

    /* Box 允许创建递归类型 */
    // 编译报错, recursive type `ListNoBox` has infinite size
    // let list = ListNoBox::Cons(1, ListNoBox::Cons(2, ListNoBox::Cons(3, Nil)));

    /* 计算非递归类型的大小 */
    /* 使用 Box<T> 给递归类型一个已知的大小 */
    let list = List::new(1, List::new(2, List::new(3, List::Nil)));
    let string = serde_json::to_string_pretty(&list).unwrap();
    println!("{}", string);
    println!();
    // {
    //   "Cons": [
    //     1,
    //     {
    //       "Cons": [
    //         2,
    //         {
    //           "Cons": [
    //             3,
    //             "Nil"
    //           ]
    //         }
    //       ]
    //     }
    //   ]
    // }

    /* 补充 */
    /* Box内存布局 */
    // 先来看看 Vec<i32> 的内存布局
    //
    // (stack)    (heap)
    // ┌──────┐   ┌───┐
    // │ vec1 │──→│ 1 │
    // └──────┘   ├───┤
    //            │ 2 │
    //            ├───┤
    //            │ 3 │
    //            ├───┤
    //            │ 4 │
    //            └───┘
    //
    // 再看看 Vec<Box<i32>> 的内存布局
    //                    (heap)
    // (stack)    (heap)   ┌───┐
    // ┌──────┐   ┌───┐ ┌─→│ 1 │
    // │ vec2 │──→│B1 │─┘  └───┘
    // └──────┘   ├───┤    ┌───┐
    //            │B2 │───→│ 2 │
    //            ├───┤    └───┘
    //            │B3 │─┐  ┌───┐
    //            ├───┤ └─→│ 3 │
    //            │B4 │─┐  └───┘
    //            └───┘ │  ┌───┐
    //                  └─→│ 4 │
    //                     └───┘

    /* 避免栈上数据的拷贝 */
    let arr = [0; 1000]; //在栈上创建一个长度为1000的数组
    let arr_copy = arr; //arr的内存分配在栈上，所以这里发生一次深拷贝
    println!("ori arr len: {}", arr.len()); //1000
    println!("after copy, arr_copy len: {}", arr_copy.len()); //1000
    let boxed_arr = Box::new([0; 1000]); //在堆上创建一个长度为1000的数组，并用一个智能指针指向它
    let boxed_arr_move = boxed_arr; //发生所有权转移，boxed_arr不再拥有所有权。此时仅拷贝了智能指针
                                    //println!("ori boxed_arr len: {}", boxed_arr.len()); //报错 borrow of moved value: `boxed_arr`
    println!("after move, boxed_arr_move len: {}", boxed_arr_move.len()); //1000
    println!();

    /* Box::leak 突破了悬空指针的限制 */
    let s = gen_static_ref(String::from("hello rust"));
    println!("Box::leak  {}", s); // hello rust
    s.push_str(", it's time to give up");
    println!("Box::leak  {}", s); // hello rust, it's time to give up
    let i = gen_static_ref(41);
    *i += 1;
    println!("Box::leak  {}", i); // 42
}
/*
/// 递归类型, 编译时不确定该分配多少内存
enum ListNoBox<T> {
    Cons(T, ListNoBox<T>),
    Nil,
}
*/

#[derive(Serialize)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
impl<T> List<T> {
    fn new(val: T, next: List<T>) -> Self {
        List::Cons(val, Box::new(next))
    }
}

enum Message {
    Quit,                       // 不需要内存空间
    Move { x: i32, y: i32 },    // 2个i32, 即 8 bytes
    Write(String),              // String也是智能指针，分配一个指针大小的内存空间即可
    ChangeColor(i32, i32, i32), // 3个i32, 即 12 bytes
}

/// 在运行期，把一个 String 类型，变成一个 'static(如果把'a替换成'static) 生命周期的 &str 类型
fn gen_static_ref<'a, T>(val: T) -> &'a mut T {
    let x = Box::new(val);
    let leak = Box::leak(x);
    leak
}
