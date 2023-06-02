#![allow(dead_code)]
#![allow(unused_variables)]

/// 3.2 数据类型
///
/// cargo r --bin 3_2
///
/// ## 目录
/// ### 标量类型
/// #### 浮点类型
/// #### 数字运算
/// #### 布尔类型
/// #### 字符类型
/// ### 复合类型
/// #### 元组类型
/// #### 数组类型
/// #### 访问数组元素
/// #### 无效的数组元素访问
///
fn main() {
    /************* 标量类型(scalar type) ************/
    // 整数类型
    /*
    长度	  有符号类型	无符号类型    表示范围
    8位     i8        u8        i: -(2^(n-1)) ~ (2^(n-1) - 1)
    16位    i16       u16       u: 0 ~ (2^(n) - 1)
    32位    i32       u32
    64位    i64       u64
    128位   i128      u128
    arch    isize     usize

    数字字面量       示例
     十进制         98_222
    十六进制        0xff
     八进制         0o77
     二进制         0b1111_0000
    字节(仅限于 u8)   b'A'

    整数溢出:
    使用 wrapping_*    : 方法在所有模式下进行包裹，例如 wrapping_add
    使用 checked_*     : 方法时发生溢出，则返回 None 值
    使用 overflowing_* : 方法返回该值和一个指示是否存在溢出的布尔值
    使用 saturating_*  : 方法使值达到最小值或最大值
     */

    // 浮点类型
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 数字运算
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
                         // remainder
    let remainder = 43 % 5;

    // 布尔类型  1个字节
    let t = true;
    let f: bool = false; // with explicit type annotation

    // 字符类型  4个字节
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /************* 复合类型(compound type) ************/
    // 元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 数组类型
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];
}
