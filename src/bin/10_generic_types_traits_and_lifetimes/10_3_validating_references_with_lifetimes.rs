use std::fmt::Display;

/// 10.3 生命周期与引用有效性
///
/// cargo r --bin 10_3
///
/// # 目录:
/// ## 生命周期与引用有效性
/// - Rust 中的每一个引用都有其 生命周期（lifetime）
/// ### 生命周期避免了悬垂引用
/// - 作用域越大我们就说它 “存在的越久”
/// #### 借用检查器
/// - 数据的生命周期必须 >= 引用的生命周期
/// ### 函数中的泛型生命周期
/// ### 生命周期标注语法
/// - 生命周期标注并不改变任何引用的生命周期的长短
/// - 生命周期参数名称必须以撇号（'）开头，其名称通常全是小写，类似于泛型其名称非常短
/// - 'a 是大多数人默认使用的名称
/// - 生命周期参数标注位于引用的 & 之后
/// ```rust
/// &i32        // 引用
/// &'a i32     // 带有显式生命周期的引用
/// &'a mut i32 // 带有显式生命周期的可变引用
/// ```
/// ### 函数签名中的生命周期标注
/// - 生命周期标注需要声明在函数名和参数列表间的尖括号中
/// ### 深入理解生命周期
/// - 指定生命周期参数的正确方式依赖函数实现的具体功能
/// ### 结构体定义中的生命周期标注
/// - 类似于泛型
/// ### 生命周期省略（Lifetime Elision）
/// - 生命周期省略规则（lifetime elision rules）: 被编码进 Rust 引用分析的模式
/// ```
/// 1、每一个是引用的参数都有它自己的生命周期参数，即每一个引用都有生命周期参数
///    fn foo<'a>(x: &'a i32)
///    fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
/// 2、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
///    fn foo<'a>(x: &'a i32) -> &'a i32
/// 3、如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self(即方法)，那么所有输出生命周期参数被赋予 self 的生命周期
/// ```
/// - 输入生命周期（input lifetimes）: 即函数或方法的参数的生命周期
/// - 输出生命周期（output lifetimes）: 即返回值的生命周期
/// ### 方法定义中的生命周期标注
/// ### 静态生命周期
/// - 'static，其生命周期能够存活于整个程序期间
/// ### 结合泛型类型参数、trait bounds 和生命周期
fn main() {
    // 生命周期避免了悬垂引用
    {
        let r;

        {
            let x = 5; // `x` does not live long enough
            r = &x; //  borrowed value does not live long enough
        } // `x` dropped here while still borrowed

        //println!("r: {}", r); //放开后编译时报错
    }

    // 借用检查器（borrow checker）
    { // r的生命周期 > x的生命周期，即引用的生命周期 > 数据的生命周期
        let r;                // ---------+-- 'a
                                    //          |
        {                           //          |
            let x = 5;         // -+-- 'b  |
            r = &x;                 //  |       |
        }                           // -+       |
                                    //          |
        //println!("r: {}", r);     //          |
    }                               // ---------+
    { // r的生命周期 < x的生命周期，即引用的生命周期 < 数据的生命周期
        let x = 5;            // ----------+-- 'b
                                   //           |
        let r = &x;          // --+-- 'a  |
                                   //   |       |
        println!("r: {}", r);      //   |       |
                                   // --+       |
    }                              // ----------+

    // 函数中的泛型生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result); // abcd

    // 生命周期标注语法
    /*
     *    &i32        // 引用
     *    &'a i32     // 带有显式生命周期的引用
     *    &'a mut i32 // 带有显式生命周期的可变引用
     */

    // 函数签名中的生命周期标注
    let str1 = String::from("x");
    let str2 = String::from("y");
    let longest_str = longest(&str1, str2.as_str());
    println!("now longest_str is: {}", longest_str); // y

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        // result的生命周期应该等于 <= String2的生命周期，尽管string2相比string1并不是最长的
        let result = longest(string1.as_str(), string2.as_str());
        // 这里的result完全在string2的生命周期范围内，所以result这个引用是安全有效的
        println!("The longest string is: {}", result);
    } // 在这里string2数据被释放

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result的生命周期还是 <= string2的
        result = longest(string1.as_str(), string2.as_str());
    }// 在这里string2数据被释放，result成悬垂引用
    //println!("The longest string is {}", result); //编译报错，`string2` does not live long enough

    // 深入理解生命周期
    let first_longest = first_longest("first", "second is longest");
    println!("first_longest: {}", first_longest); // first

    // 结构体定义中的生命周期标注
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let excerpt = ImportantExcerpt::new(first_sentence);
    excerpt.print(); // part: 'Call me Ishmael', 15 bytes.

    // 生命周期省略（Lifetime Elision）
    // 方法定义中的生命周期标注
    let default = "5";
    let new_level = excerpt.announce_and_return_part(default);
    println!("new level is {}", new_level); // Call me Ishmael

    // 静态生命周期
    //这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面量都是 'static 的
    let s: &'static str = "I have a static lifetime.";

    // 结合泛型类型参数、trait bounds 和生命周期
    let announcement = longest_with_an_announcement("first", "second", 45);
    println!("{}", announcement); // second

    println!("{}", true > false);   // true
    println!("{}", true > true);    // false
    println!("{}", false > true);   // false
    println!("{}", false > false);  // false
    println!("{}", true == true);   // true
    println!("{}", false == false); // true
}
// longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致
// 通过在函数签名中指定生命周期参数时，我们并没有改变任何传入值或返回值的生命周期，而是指出任何不满足这个约束条件的值都将被借用检查器拒绝
// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分
// 即泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
// 故返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 总是返回第一个参数而不是最长的字符串 slice，此时就不需要为参数 y 指定一个生命周期
fn first_longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// 悬垂引用
// cannot return reference to local variable `result`
/*
fn non_longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
 */
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn new(part: &'a str) -> Self {
        ImportantExcerpt { part }
    }
    fn print(&self) {
        println!("part: '{}', {} bytes.", self.part, self.part.len());
    }
    // 这里的level就不需要显示地申明生命周期标注，因为&self.
    fn announce_and_return_part(&self, level: &str) -> &str {
        println!("Attention please: {}", level);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}