#![allow(dead_code)]
#![allow(unused_variables)]

/// 19.5 宏
///
/// cargo r --bin 19_5
///
/// ## 目录:
/// - 声明（Declarative）宏： macro_rules!
/// - 过程（Procedural）宏：
///   1、自定义派生: #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
///   2、类属性:（Attribute-like）宏定义可用于任意项的自定义属性
///   3、类函数: 宏看起来像函数不过作用于作为参数传递的 token
/// ### 宏和函数的区别
/// - 个函数标签必须声明函数参数个数和类型，而宏能够接受不同数量的参数
/// - 宏可以在一个给定类型上实现 trait，而函数不行
/// - 实现一个宏比函数要复杂
/// - 在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用
/// ### 使用 macro_rules! 的声明宏用于通用元编程
/// - 声明宏允许我们编写一些类似 Rust match 表达式的代码
/// ### 用于从属性生成代码的过程宏
/// ### 如何编写自定义 derive 宏
/// ### 类属性宏
/// ### 类函数宏
///
fn main() {
    // 宏和函数的区别
    // 使用 macro_rules! 的声明宏用于通用元编程
    let v: Vec<u32> = vec![1, 2, 3];
    /*#[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }*/

    // 用于从属性生成代码的过程宏

    // 如何编写自定义 derive 宏

    // 类属性宏
    /*
    #[route(GET, "/")]
    fn index() {}

    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
    */

    // 类函数宏
    /*
    let sql = sql!(SELECT * FROM posts WHERE id=1);

    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {}
    */
}
