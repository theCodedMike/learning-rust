type Thunk = Box<dyn Fn() + Send + 'static>;

/// 19.3 高级类型
///
/// cargo r --bin 19_3
///
/// ## 目录:
/// ### 为了类型安全和抽象而使用 newtype 模式
/// ### 类型别名用来创建类型同义词
/// - 类型别名的主要用途是减少重复
/// ### 从不返回的 never type
/// - !类型，被称为 empty type，因为它没有值。我们更倾向于称之为 never type
/// - 在函数从不返回的时候充当返回值
/// - 从不返回的函数被称为 发散函数（diverging functions）
/// - never type 可以强转为任何其他类型
/// ### 动态大小类型和 Sized trait
/// - 动态大小类型（dynamically sized types）：有时被称为 “DST” 或 “unsized types”，这些类型允许我们处理只有在运行时才知道大小的类型
/// - str   字符串切片    &str包含str的地址和长度
/// - trait  特征也是动态大小类型     &dyn Trait  Box<dyn Trait>
/// - [T]   数组切片
///
fn main() {
    // 为了类型安全和抽象而使用 newtype 模式
    // 类型别名用来创建类型同义词
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f = returns_long_type();
    takes_long_type(f);

    // 从不返回的 never type
    fn bar() -> ! {
        // 发散函数
        // 以下这3种方式的返回值都是!
        //todo!();
        //panic!("panic");
        loop {}
    }

    // 动态大小类型和 Sized trait
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
