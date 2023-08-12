#![allow(dead_code)]
#![allow(unused_variables)]

use std::str::FromStr;

type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

/// 19.3 高级类型
///
/// cargo r --bin advanced-types
///
/// ## 目录
/// ### 为了类型安全和抽象而使用 newtype 模式
///
/// ### 类型别名(type alias)用来创建类型同义词
/// - 类型别名的主要用途是减少重复
///
/// ### 从不返回的 never type
/// - !类型，被称为 empty type，因为它没有值。我们更倾向于称之为 never type
/// - 在函数从不返回的时候充当返回值
/// - 从不返回的函数被称为 发散函数(diverging functions)
/// - never type 可以强转为任何其他类型
///
/// ### 动态大小类型(dynamically sized types / unsized types)和 Sized trait
/// - 动态大小类型(dynamically sized types): 有时被称为 "DST" 或 "unsized types"，这些类型允许我们处理只有在运行时才知道大小的类型
/// - 为了处理 DST，Rust 提供了 Sized trait 来决定一个类型所占用的内存大小是否在编译时可知
/// - 对于DST，使用智能指针(例如Box)将它转换为指针即可
/// - ?Sized 表示编译时占用的内存大小不确定
///
///   str              字符串切片    &str包含str的地址和长度
///   trait object     特征对象      &dyn Trait / Box<dyn Trait>
///   [T]              数组切片      &[T]
///   递归类型                       Box
///
fn main() {
    /* 为了类型安全和抽象而使用 newtype 模式 */

    /* 类型别名用来创建类型同义词 */
    type Kilometers = i32; // Kilometers 是 i32 的 同义词(synonym)
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y); // 10
    let f = returns_long_type();
    takes_long_type(f);
    println!();

    /* 从不返回的 never type */
    // bar();

    /* 动态大小类型和 Sized trait */
    generic(4);
    generic_unsized(&[10]);
    println!();

    /* 补充 */
    // as 转换
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8; // 将字符'a'转换为整数，97
    let d = 97 as char; // only `u8` can be cast into `char`
    println!("{} {} {} {}", a, b, c, d); // 3 100 97 a

    // 内存地址转换为指针
    let mut nums = [1, 2];
    let p_1 = nums.as_mut_ptr();
    println!("{:?}", unsafe { *p_1 }); // 1
    let p_usize = p_1 as usize;
    let second = p_usize + 4;
    let p_2 = second as *mut i32;
    println!("{:?}", unsafe { *p_2 }); // 2
    unsafe {
        *p_2 += 10;
    }
    println!("{:?}", unsafe { *p_2 }); // 12

    // as 转换不具有传递性，即 a as U1 as U2 是合法的，不代表 a as U2 是合法的

    // 以下2个Trait可以实现类型转换，实现其中的1个即可，
    // From/Into  不支持处理异常
    // TryFrom/TryInto  支持处理异常
    let s = "hello";
    let s1 = String::from(s);
    //let s1: String = s.into();
    let a = 15;
    //let a = 1500;
    let b = match u8::try_from(a) {
        Ok(val) => val,
        Err(e) => panic!("Failed to convert: {}", e),
    }; // Failed to convert: out of range integral type conversion attempted
    println!("Successfully to convert: {}", b);
    let i = "5".parse::<i32>().unwrap();
    let i = i32::from_str("5").unwrap();
    let s = "hello";
    let box_str: Box<str> = s.into();
    println!(
        "{}, {} bytes, {}",
        box_str,
        std::mem::size_of_val(&box_str),
        std::mem::size_of_val(&s)
    ); // hello, 16 bytes, 16
    println!();

    // Transmute 转变
    //std::mem::transmute 将一种类型的值的位重新解释为另一种类型, Both types must have the same size.
    //std::mem::transmute_copy
    let pointer = foo as *const ();
    let function = unsafe {
        //Turning a raw pointer into a function pointer.
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    println!("{}", function()); // 0
    let a = 5;
    let r = R(&a);
    let r1 = unsafe { extend_lifetime(r) };
    println!("{:?}", r1); // R(5)
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

/// T: Sized 表示T所占内存空间的大小必须在编译期确定
fn generic<T: Sized>(t: T) {
    // --snip--
}

/// T: ?Sized 表示T可能是也可能不是 Sized，即编译时所占内存大小不确定也可
///
/// ?Trait 语法只能用于Sized ，而不能用于任何其他trait
///
fn generic_unsized<T: ?Sized>(t: &T) {
    // --snip--
}

fn foo() -> i32 {
    0
}

#[derive(Debug)]
struct R<'a>(&'a i32);
unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    // extending a lifetime
    std::mem::transmute::<R<'b>, R<'static>>(r)
}
unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
    // shortening an invariant lifetime
    std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
}
