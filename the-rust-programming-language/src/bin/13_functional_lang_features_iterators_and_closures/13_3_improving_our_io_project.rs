/// 13.3 改进之前的IO项目
///
/// cargo r --bin io-project
///
/// ## 目录:
/// ### 使用迭代器并去掉 clone
/// #### 直接使用 env::args 返回的迭代器
/// #### 使用 Iterator trait 代替索引
/// ### 使用迭代器适配器来使代码更简明
///
fn main() {
    let x: bool = Default::default();
    println!("{}", x); // false
    let string: String = Default::default();
    println!("{}", string);
    /*
    { (), (), "Returns the default value of `()`" }
    { bool, false, "Returns the default value of `false`" }
    { char, '\x00', "Returns the default value of `\\x00`" }

    { usize, 0, "Returns the default value of `0`" }
    { u8, 0, "Returns the default value of `0`" }
    { u16, 0, "Returns the default value of `0`" }
    { u32, 0, "Returns the default value of `0`" }
    { u64, 0, "Returns the default value of `0`" }
    { u128, 0, "Returns the default value of `0`" }

    { isize, 0, "Returns the default value of `0`" }
    { i8, 0, "Returns the default value of `0`" }
    { i16, 0, "Returns the default value of `0`" }
    { i32, 0, "Returns the default value of `0`" }
    { i64, 0, "Returns the default value of `0`" }
    { i128, 0, "Returns the default value of `0`" }

    { f32, 0.0f32, "Returns the default value of `0.0`" }
    { f64, 0.0f64, "Returns the default value of `0.0`" }
     */
}
