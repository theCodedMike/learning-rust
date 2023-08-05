#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// 10.3 使用生命周期来确保引用有效
///
/// cargo r --bin lifetime
///
/// ## 目录
/// Rust 中的每一个引用都有其 生命周期(lifetime)
/// ### 生命周期避免了悬垂引用
/// - 作用域越大我们就说它 "存在的越久"
/// #### 借用检查器
/// - 数据的生命周期必须 >= 引用的生命周期
///
/// ### 函数中的泛型生命周期
///
/// ### 生命周期标注语法
/// - 生命周期标注并不改变任何引用的生命周期的长短
/// - 生命周期参数名称必须以单引号(')开头，其名称通常全是小写，类似于泛型其名称非常短
/// - 'a 是大多数人默认使用的名称
/// - 生命周期参数标注位于引用的 & 之后
/// ```rust
/// &i32        // 引用
/// &'a i32     // 带有显式生命周期的引用
/// &'a mut i32 // 带有显式生命周期的可变引用
/// ```
/// ### 函数签名中的生命周期标注
/// - 生命周期标注需要声明在函数名和参数列表间的尖括号中
///
/// ### 深入理解生命周期
/// - 指定生命周期参数的正确方式依赖函数实现的具体功能
///
/// ### 结构体定义中的生命周期标注
/// - 类似于泛型
///
/// ### 生命周期省略(Lifetime Elision)
/// - 生命周期省略规则(lifetime elision rules): 被编码进 Rust 引用分析的模式
/// ```
/// 1、每一个是引用的参数都有它自己的生命周期参数，即每一个引用都有生命周期参数
///    fn foo<'a>(x: &'a i32)
///    fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
/// 2、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
///    fn foo<'a>(x: &'a i32) -> &'a i32
/// 3、如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self(即方法)，那么所有输出生命周期参数被赋予 self 的生命周期
/// ```
/// - 输入生命周期(input lifetimes): 即函数或方法的入参的生命周期
/// - 输出生命周期(output lifetimes): 即返回值的生命周期
///
/// ### 方法定义中的生命周期标注
///
/// ### 静态生命周期
/// - 'static，其生命周期能够存活于整个程序期间
///
/// ### 结合泛型类型参数、trait bounds 和生命周期
///
fn main() {
    /* 生命周期避免了悬垂引用 */
    {
        let r;

        {
            let x = 5; // `x` does not live long enough
            r = &x; //  borrowed value does not live long enough
        } // `x` dropped here while still borrowed

        //println!("r: {}", r); //放开后编译时报错
    }

    // 借用检查器(borrow checker)
    //
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); // ---------+
    //
    // r的生命周期 < x的生命周期，即引用的生命周期 < 数据的生命周期，而上例中恰好相反

    /* 函数中的泛型生命周期 */
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result); // abcd
    println!();

    /* 生命周期标注语法 */
    //
    //   &i32        // 引用
    //   &'a i32     // 带有显式生命周期的不可变引用
    //   &'a mut i32 // 带有显式生命周期的可变引用
    //

    /* 函数签名中的生命周期标注 */
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
        println!("The longest string is: {}", result); // long string is long
    } // 在这里string2数据被释放

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result的生命周期还是 <= string2的
        result = longest(string1.as_str(), string2.as_str());
    } // 在这里string2数据被释放，result成悬垂引用
      //println!("The longest string is {}", result); //编译报错，`string2` does not live long enough
    println!();

    /* 深入理解生命周期 */
    let first_longest = first_longest("first", "second is longest");
    println!("first_longest: {}", first_longest); // first
    println!();

    /* 结构体定义中的生命周期标注 */
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt::new(first_sentence);
    excerpt.print(); // part: 'Call me Ishmael', 15 bytes.
    println!();

    /* 生命周期省略(Lifetime Elision)规则 */
    // 1: 编译器为每一个引用参数都分配一个生命周期参数
    // 2: 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
    // 3: 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self, 那么所有输出生命周期参数被赋予 self 的生命周期

    /* 方法定义中的生命周期标注 */
    let default = "5";
    let new_level = excerpt.announce_and_return_part(default);
    println!("new level is {}", new_level); // Call me Ishmael
    let new_level = excerpt.announce_and_return_part_2(default);
    println!("new level is {}", new_level); // 5
    println!();

    /* 静态生命周期 */
    //这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面量都是 'static 的
    let s: &'static str = "I have a static lifetime.";
    // 需要区分 &'static 和 T: 'static
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    ); // The 12 bytes at 0x7FF7E4DDA7C0 stored: Hello World!

    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 42;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
        // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，数据并不会被释放
    }
    println!("&'static i32: {}", r1); // 42
    println!("&'static str: {}", r2); // &'static str
    let r3: &str;
    {
        let s1 = "String".to_string();
        // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
        // 所以T: 'static可以理解为只要在T被使用的时候T没有被drop，那么它的生命周期就是'static
        // 有一种特殊情况是在整个程序运行期间T都是有效的(没有被drop)，那么此时的T: 'static等价于&'static
        // 所以T: 'static表达的范围更宽泛
        static_bound(&s1);
        // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
        //r3 = &s1; //这里报错 `s1` does not live long enough
        r3 = "&s1 isn't right here";
        // s1 在这里被 drop
    }
    println!("{}", r3); // &s1 isn't right here

    // i 是有所有权的数据，并没有包含任何引用，这里它是 'static
    let i = 5;
    print_it(i); // 此时T为i32
    print_it1(i);
    //print_it2(i); // 类型不匹配

    // 但是 &i 是一个引用，生命周期受限于作用域，因此它不是 'static
    let ref_i = &i;
    //print_it(ref_i); // 报错，此时T为&i32，borrowed value does not live long enough
    //print_it1(&i); // 报错
    print_it2(ref_i); // 没问题，此时T为i32

    // 如果是&str呢?
    let s = "hello rust";
    print_it(s); // 没问题，此时T为&str
    print_it1(s);
    print_it2(s); // 没问题，此时T为str
                  //print_it(&s); // 报错，此时T为&&str，borrowed value does not live long enough
                  //print_it1(&s); // 报错
    print_it2(&s); // 没问题，此时T为&str
    println!();

    // 结合泛型类型参数、trait bounds 和生命周期
    let announcement = longest_with_an_announcement("first", "second", 45);
    println!("{}", announcement); // second

    /* 补充 */
    /* 不太聪明的生命周期检查: 编译器不是万能的,有时候很傻 */
    let mut map = HashMap::new();
    map.insert("hello", 6);
    //let default = get_default(&mut map, "hello");
    println!("example_1: {}", default);
    println!();

    /* 无界生命周期 */
    // 不安全代码(unsafe)经常会凭空产生引用或生命周期，这些生命周期被称为是 无界(unbound) 的
    // 无界生命周期往往是在解引用一个裸指针(裸指针raw pointer)时产生的，换句话说，它是凭空产生的，因为输入参数根本就没有这个生命周期
    // fn f<'a, T>(x: *const T) -> &'a T {
    //     unsafe {
    //         &*x
    //     }
    // }
    // 参数x是一个裸指针，它并没有任何生命周期，然后通过unsafe操作后，它被进行了解引用，变成了一个Rust的标准引用类型，该类型必须要有生命周期，也就是'a
    // 在实际应用中，要尽量避免这种无界生命周期。最简单的避免无界生命周期的方式就是在函数声明中运用生命周期消除规则

    /* HRTB(Higher-Rank Trait Bounds) https://doc.rust-lang.org/nomicon/hrtb.html */
    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    println!("{}", clo.call());
    println!(); // 0

    /* 生命周期排序 */
    let single_ref = SingleRef::new(&"hello");
    println!("{:?}", single_ref); // SingleRef { r: "hello" }
    let x = 5;
    let y = 10;
    let double_ref = DoubleRef::new(&x, &y);
    println!("{:?}", double_ref); // DoubleRef { r: 5, s: 10 }
    let ref2 = DoubleRef2::new(&"rust", &3.2);
    println!("{:?}", ref2); // DoubleRef2 { r: "rust", s: 3.2 }
    println!();

    /* 闭包中的生命周期消除规则 目前无解,此时推荐使用函数而不是闭包 */
    //returning this value requires that `'1` must outlive `'2`
    //                          '1       '2
    //let closure_elision = |x: &i32| -> &i32 { x };
    fn fn_elision(x: &i32) -> &i32 {
        x
    }

    /* NLL (Non-Lexical Lifetime) */
    // 引用的生命周期正常来说应该从借用开始一直持续到作用域结束，但是这种规则会让多引用共存的情况变得非常复杂
    // rust 1.31 版本引入NLL后，就变成了引用的生命周期从借用处开始，一直持续到最后一次使用的地方
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 引入NLL后，r1, r2的作用域在这里结束
    let r3 = &mut s;
    println!("NLL: {}", r3);
    println!();

    /* Reborrow 再借用  &mut T -> &T */
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p; // 可变引用
    let rr = &*r; // reborrow! 此时对`r`的再借用不会跟上面的借用冲突
    println!("Reborrow: rr = {:?}", rr); // 再借用`rr`最后一次使用发生在这里，在它的生命周期中，我们并没有使用原来的借用`r`，因此不会报错
    r.move_to(10, 10);
    // 再借用结束后，才去使用原来的借用`r`
    println!("Reborrow: r = {:?}", r); // Reborrow: r = Point { x: 10, y: 10 }
                                       // 如果这样写就会有问题, 因为r调用move_to()方法时会同时存在可变引用与不可变引用
                                       // println!("Reborrow: r = {:?}", rr);
    println!();

    /* 生命周期消除规则补充 */
    // impl<'a> Reader for BufReader<'a> {
    //     // methods go here
    //     // impl内部实际上并没有用到'a
    // }
    // 在impl内部的方法中，根本就没有用到'a，那就可以写成下面的代码形式
    // impl Reader for BufReader<'_> {
    //     // methods go here
    // }

    /* 一个复杂的例子 */
    let mut list = List {
        manager: Manager { text: "rust" },
    };
    list.get_interface().noop(); // ---- mutable borrow occurs here
    println!("Interface should be dropped here and the borrow released");
    use_list(&list); //immutable borrow occurs here
}

/// longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致
///
/// 通过在函数签名中指定生命周期参数时，我们并没有改变任何传入值或返回值的生命周期，而是指出任何不满足这个约束条件的值都将被借用检查器拒绝
///
/// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分
///
/// 即泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
///
/// 故返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效
///
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 总是返回第一个参数而不是最长的字符串 slice，此时就不需要为参数 y 指定一个生命周期
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
    /// 这里的level就不需要显示地申明生命周期标注，因为&self.
    fn announce_and_return_part(&self, level: &str) -> &str {
        println!("Attention please: {}", level);
        self.part
    }
    fn announce_and_return_part_2<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b, //这里`a: `b表示a的生命周期比b的长
    {
        println!("Attention please: {}", announcement);
        announcement
    }
}

/// lifetime parameters must be declared prior to type and const parameters
///
/// 'a必须在T的前面
///
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// cannot borrow `*map` as mutable more than once at a time
// fn get_default<K, V>(map: &mut HashMap<K, V>, key: K) -> &mut V
// where
//     K: Clone + Eq + Hash,
//     V: Default,
// {
//     match map.get_mut(&key) {
//         // first mutable borrow occurs here
//         Some(value) => value,
//         None => {
//             map.insert(key.clone(), V::default());
//             map.get_mut(&key).unwrap() // second mutable borrow occurs here
//         }
//     }
// }

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

///
/// for<'a> can be read as " for all choices of 'a "
///
impl<F> Closure<F>
where
    // F: Fn(&(u8, u16)) -> &u8,
    // for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
    F: for<'a> Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 {
    &data.0
}

//
// r的生命周期'a必须要比T的生命周期更短, 这是rust 1.31版本之前的写法
// struct Ref<'a, T: 'a> {
//    r: &'a T
// }
//
#[derive(Debug)]
struct SingleRef<'a, T> {
    //Rust 1.31 版本后可以简写为这样
    r: &'a T,
}

impl<'a, T> SingleRef<'a, T> {
    fn new(r: &'a T) -> Self {
        SingleRef { r }
    }
}

/// 越靠前,生命周期越短
#[derive(Debug)]
struct DoubleRef<'a, 'b: 'a, T> {
    //'b至少活得跟'a一样久  'b >= 'a
    r: &'a T,
    s: &'b T,
}

impl<'a, 'b, T> DoubleRef<'a, 'b, T> {
    fn new(r: &'a T, s: &'b T) -> Self {
        DoubleRef { r, s }
    }
}

#[derive(Debug)]
struct DoubleRef2<'a, T, M> {
    r: &'a T,
    s: &'a M,
}

impl<'a, T, M> DoubleRef2<'a, T, M> {
    fn new(r: &'a T, s: &'a M) -> Self {
        DoubleRef2 { r, s }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

/// 这种方案是有问题的，生命周期声明过大
///
/// struct Interface<'a> {
///     manager: &'a mut Manager<'a>
/// }
///
/// impl<'a> Interface<'a> {
///     pub fn noop(self) {
///         println!("interface consumed");
///     }
/// }
///
/// struct List<'a> {
///     manager: Manager<'a>,
/// }
///
/// impl<'a> List<'a> {
///     pub fn get_interface(&'a mut self) -> Interface {
///         Interface {
///             manager: &mut self.manager
///         }
///     }
/// }
struct Manager<'a> {
    text: &'a str,
}

struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>,
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
    where
        'a: 'b,
    {
        Interface {
            manager: &mut self.manager,
        }
    }
}
fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

fn get_memory_location() -> (usize, usize) {
    // "Hello World" 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量(owner)的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
    let owner = "Hello World!";
    let pointer = owner.as_ptr() as usize;
    let length = owner.len();
    (pointer, length)
    // `owner` 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(pointer as *const u8, length))
    }
}

fn static_bound<T: Display + 'static>(t: &T) {
    println!("static_bound: {}", t);
}

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}
fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}
fn print_it2<T: Debug + 'static + ?Sized>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}
