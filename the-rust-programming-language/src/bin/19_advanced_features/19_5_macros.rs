#![allow(dead_code)]
#![allow(unused_variables)]

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

/// 19.5 宏
///
/// cargo r --bin macros
///
/// ## 目录
/// - 声明宏(declarative macros): macro_rules!
/// - 过程宏(procedural macros):
///   1. 自定义派生宏: #[derive()] 宏在结构体和枚举上指定通过 derive 属性添加的代码
///   2. 类属性宏: (Attribute-like)宏定义可用于任意项的自定义属性
///   3. 类函数宏: 宏看起来像函数不过作用于作为参数传递的 token
/// ### 宏和函数的区别
/// - 宏是一种为写其他代码而写代码的方式，即所谓的 元编程(metaprogramming)
/// - 一个函数签名必须声明函数参数个数和类型，而宏能够接受不同数量的参数
/// - 宏可以在一个给定类型上实现 trait，而函数不行
/// - 实现一个宏比实现一个函数要复杂
/// - 在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用
///
/// ### 使用 macro_rules! 的声明宏用于通用元编程
/// - 声明宏允许我们编写一些类似 Rust match 表达式的代码
///
/// ### 用于从属性生成代码的过程宏
///
/// ### 如何编写自定义 derive 宏(custom derive macro)
/// - derive过程宏只能用在struct/enum/union上，多数用在结构体上
///
/// ### 类属性宏(attribute-like macros)
/// - 类属性宏除了可以用于struct/enum/union外，还可以用于其它项，例如函数
///
/// ### 类函数宏(function-like macros)
///
fn main() {
    /* 宏和函数的区别 */

    /* 使用 macro_rules! 的声明宏用于通用元编程 */
    let v: Vec<u32> = vec![1, 2, 3];
    // #[macro_export]
    // macro_rules! vec {
    //     ( $( $x:expr ),* ) => {
    //         {
    //             let mut temp_vec = Vec::new();
    //             $(
    //                 temp_vec.push($x);
    //             )*
    //             temp_vec
    //         }
    //     };
    // }

    /* 用于从属性生成代码的过程宏 */

    /* 如何编写自定义 derive 宏 */
    // 见项目 rust_macro/hello_macro
    // std提供的可派生的Trait:
    // 1、用于程序员输出的 Debug
    // 2、等值比较的 PartialEq 和 Eq
    // 3、次序比较的 PartialOrd 和 Ord
    // 4、复制值的 Clone 和 Copy
    // 5、固定大小的值到值映射的 Hash
    // 6、默认值的 Default
    Pancakes::hello_macro(); // Hello, Macro! My name is Pancakes!
    User::hello_macro(); // Hello, Macro! My name is User!

    /* 类属性宏 */
    // #[route(GET, "/")]
    // fn index() {}
    //
    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    //     // 第一个参数是说明属性包含的内容: Get, "/" 部分
    //     // 第二个是属性所标注的类型项，在这里是 fn index() {...}，注意，函数体也被包含其中
    // }
    //

    /* 类函数宏 */
    // 这个宏会解析其中的 SQL 语句并检查其是否正确
    // let sql = sql!(SELECT * FROM posts WHERE id=1);
    //
    // sql!的定义如下:
    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {}
}

#[derive(HelloMacro)]
struct Pancakes;
#[derive(HelloMacro)]
struct User {
    name: String,
    age: u16,
}
