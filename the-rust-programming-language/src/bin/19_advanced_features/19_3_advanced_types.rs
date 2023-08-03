#![allow(dead_code)]
#![allow(unused_variables)]

type Thunk = Box<dyn Fn() + Send + 'static>;

/// 19.3 高级类型
///
/// cargo r --bin advanced-types
///
/// ## 目录
/// ### 为了类型安全和抽象而使用 newtype 模式
///
/// ### 类型别名用来创建类型同义词
/// - 类型别名的主要用途是减少重复
///
/// ### 从不返回的 never type
/// - !类型，被称为 empty type，因为它没有值。我们更倾向于称之为 never type
/// - 在函数从不返回的时候充当返回值
/// - 从不返回的函数被称为 发散函数（diverging functions）
/// - never type 可以强转为任何其他类型
///
/// ### 动态大小类型(dynamically sized types / unsized types)和 Sized trait
/// - 动态大小类型（dynamically sized types）：有时被称为 "DST" 或 "unsized types"，这些类型允许我们处理只有在运行时才知道大小的类型
/// - 为了处理 DST，Rust 提供了 Sized trait 来决定一个类型的大小是否在编译时可知
/// - str   字符串切片    &str包含str的地址和长度
/// - trait  特征也是动态大小类型     &dyn Trait  Box<dyn Trait>
/// - [T]   数组切片
///
fn main() {
    /* 为了类型安全和抽象而使用 newtype 模式 */

    /* 类型别名用来创建类型同义词 */
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f = returns_long_type();
    takes_long_type(f);

    /* 从不返回的 never type */
    // bar();

    /* 动态大小类型和 Sized trait */
    generic(4);
    generic_unsized(&[10]);
}

fn takes_long_type(f: Thunk) {
    // --snip--
    println!("takes_long_type");
    f();
}

fn returns_long_type() -> Thunk {
    // --snip--
    println!("returns_long_type");
    Box::new(|| println!("hi from returns_long_type"))
}

/// 发散函数
fn bar() -> ! {
    // 以下这4种方式的返回值都是!
    //
    // todo!(); // 1
    // unimplemented!(); // 2
    // panic!("here will panic"); // 3
    loop {} // 4
}

/// T: Sized 表示T的大小必须能在编译期确定
fn generic<T: Sized>(t: T) {
    // --snip--
}

/// T: ?Sized 表示T可能是也可能不是 Sized
///
/// ?Trait 语法只能用于Sized ，而不能用于任何其他trait
///
fn generic_unsized<T: ?Sized>(t: &T) {
    // --snip--
}
