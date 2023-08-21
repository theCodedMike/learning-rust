#![allow(dead_code)]
#![allow(unused_variables)]

use num::Complex;

/// 3.2 数据类型
///
/// cargo r --bin data-types
///
/// ## 目录
/// ### 标量类型
/// #### 浮点类型
/// #### 整数运算
/// 长度	  有符号类型	无符号类型    表示范围
///  8位     i8        u8        i: -(2^(n-1)) ~ (2^(n-1) - 1)
///  16位    i16       u16       u: 0 ~ (2^(n) - 1)
///  32位    i32       u32
///  64位    i64       u64
///  128位   i128      u128
///  arch    isize     usize
///
///  数字字面量       示例
///   十进制         98_222
///  十六进制        0xff
///   八进制         0o77
///   二进制         0b1111_0000
///  字节(仅限于 u8)   b'A'
///
///  整数溢出:
///  使用 wrapping_*    : 方法在所有模式下进行包裹，例如 wrapping_add
///  使用 checked_*     : 方法时发生溢出，则返回 None 值
///  使用 overflowing_* : 方法返回该值和一个指示是否存在溢出的布尔值
///  使用 saturating_*  : 方法使值达到最小值或最大值
/// #### 布尔类型
/// #### 字符类型
///
/// ### 复合类型
/// #### 元组类型
/// #### 数组类型
/// #### 访问数组元素
/// #### 无效的数组元素访问
///
fn main() {
    /* 类型推导和标注 */
    let f64_val = 4.2;
    //let guess = "42".parse().expect("Not a number!"); //  type annotations needed
    let guess_a: i32 = "42".parse().expect("Not a number!");
    let guess_b = "42".parse::<i32>().expect("Not a number!");
    println!("{} {}", guess_a, guess_b); // 42 42
    println!();

    /************* 标量类型(scalar type) ************/
    /* 整数类型 */
    let a: u8 = 255;
    let b = 98_222; //十进制表示
    let c = 0xff; //十六进制表示
    let d = 0o77; //八进制表示
    let e = 0b1001_0000; //二进制表示
    println!("{} {} {} {} {}", a, b, c, d, e); // 255 98222 255 63 144
    println!("i32: {} bytes", std::mem::size_of_val(&i32::MAX)); // 4
    println!();

    /* 浮点类型 */
    let y: f32 = 3.0; // f32
    let x = 4.0_f64; // f64
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!("   0.1 + 0.2: {:#b}", (abc.0 + abc.1).to_bits()); // 0b111110100110011001100110011010
    println!("         0.3: {:b}", (abc.2).to_bits()); //            111110100110011001100110011010
    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:#x}", (xyz.0 + xyz.1).to_bits()); // 0x3fd3333333333334
    println!("         0.3: {:#X}", (xyz.2).to_bits()); //         0x3FD3333333333333
    assert_eq!(abc.0 + abc.1, abc.2);
    //assert_eq!(xyz.0 + xyz.1, xyz.2);
    // 错误  left: `0.30000000000000004`, right: `0.3`
    println!("f32: {} bytes", std::mem::size_of_val(&f32::MAX)); // 8
    println!("f64: {} bytes", std::mem::size_of_val(&f64::MIN)); // 4
    println!();

    /* 数字运算 */
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
    let a: u8 = 255;
    let wrapping_result = a.wrapping_add(20);
    let checked_result = a.checked_add(20);
    let overflowing_result = a.overflowing_add(20);
    let saturating_result = a.saturating_add(20);
    println!("数值类型溢出: wrapping_result = {}", wrapping_result); // 19
    println!("数值类型溢出: checked_result = {:?}", checked_result); // None
    println!(
        "数值类型溢出: overflowing_result = {:?}",
        overflowing_result
    ); // (19, true)
    println!("数值类型溢出: saturating_result = {}", saturating_result); // 255
    println!();

    /* 布尔类型  1个字节 */
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!(
        "bool: {} is {} bytes",
        bool::default(),
        std::mem::size_of_val(&bool::default())
    ); // false is 1 bytes
    println!("{}", true > false); // true
    println!("{}", true > true); // false
    println!("{}", false > true); // false
    println!("{}", false > false); // false
    println!("{}", true == true); // true
    println!("{}", false == false); // true
    println!();

    /* 字符类型  4个字节 */
    let c = 'z';
    let z = 'ℤ';
    let u_z = z as u8;
    println!("u_z: {}, u_c: {}", u_z, c as u32); // 36 122
    let heart_eyed_cat = '😻';
    println!("u_cat: {}", heart_eyed_cat as u32); // 128571
    println!(" 'z': {} bytes", std::mem::size_of_val(&c)); // 4 bytes
    println!("'😻': {} bytes", std::mem::size_of_val(&heart_eyed_cat)); // 4 bytes
    println!();

    /* NaN 跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较 */
    let nan = f64::NAN; // 2 / 0.0 is NaN
    let x = (-4.0_f32).sqrt();
    // assert_eq!(x, x);
    if x.is_nan() {
        println!("x is NaN");
    }
    println!();

    /************* 复合类型(compound type) ************/
    /* 元组类型 */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y); // 6.4
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(" tup(i32, f64, u8): {} bytes", std::mem::size_of_val(&x)); //  16 bytes
    let unit_ele = (2, 3, 5);
    println!(
        "size of (2, 3, 4) is {} bytes.",
        std::mem::size_of_val(&unit_ele)
    ); // (2, 3, 4) is 12 bytes
    let s1 = String::from("hello");
    let (s2, len) = calculate_len(s1);
    println!("the length of '{}' is {}", s2, len); // the length of 'hello' is 5
    println!();

    /* 数组类型 [T; n] */
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];
    //let six = a[5]; // panic, index out of bounds
    println!("arr[i32; 5]: {} bytes", std::mem::size_of_val(&a)); //  20 bytes

    let str_arr = std::array::from_fn::<_, 5, _>(|i| "array".to_string());
    println!("{:?}", str_arr); // ["array", "array", "array", "array"]
    println!("{:?}", str_arr.get(5)); // None, 通过get方法越界访问数组，并不会panic

    let one = [1, 2, 3];
    let two = [0; 3];
    //对于二维数组, 其一维数组的长度必须相等; 对于动态数组(vec)则没有这个限制
    let arr_2d = [one, two];
    println!("{:?}", arr_2d); // [[1, 2, 3], [0, 0, 0]]
    println!();

    /*************补充*************/
    /* 位运算 */
    let binary_a = 2; // 二进制为0000_0010
    let binary_b = 3; // 二进制为0000_0011
    println!("a: {:08b}", binary_a);
    println!("b: {:08b}", binary_b);
    println!(
        "(a & b) = {} -> {:08b}",
        binary_a & binary_b,
        (binary_a & binary_b)
    ); // (a & b) = 2 -> 00000010
    println!(
        "(a | b) = {} -> {:08b}",
        binary_a | binary_b,
        binary_a | binary_b
    ); // (a | b) = 3 -> 00000011
    println!(
        "(a ^ b) = {} -> {:08b}",
        binary_a ^ binary_b,
        binary_a ^ binary_b
    ); // (a ^ b) = 1 -> 00000001
    println!("(!b) = {} : {:08b}", !binary_b, !binary_b);
    // (!b) = -4 : 11111111111111111111111111111100
    println!(
        "(a << b) = {} -> {:08b}",
        binary_a << binary_b,
        binary_a << binary_b
    ); // (a << b) = 16 -> 00010000
    println!(
        "(a >> b) = {} : {:08b}",
        binary_a >> binary_b,
        binary_a >> binary_b
    ); // (a >> b) = 0 : 00000000
    let mut new_a = binary_a;
    new_a <<= binary_b;
    println!("(a << b) = {}", new_a); // (a << b) = 16
    println!();

    /* 范围 只允许用于数字或字符类型 */
    print!("Range Integer: ");
    for i in 1..=10 {
        print!("{} ", i); // 1 2 3 4 5 6 7 8 9 10
    }
    print!("\nRange Character: ");
    for j in 'A'..='Z' {
        print!("{} ", j); // A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
    }
    println!("\n");

    /* 有理数和复数  需要使用三方包`num` */
    let cp_a = Complex { re: 2.1, im: -1.2 };
    let cp_b = Complex::new(11.1, 22.2);
    let result = cp_a + cp_b;
    println!("{} + {}i", result.re, result.im); // 13.2 + 21i
    println!("{}", result); // 13.2+21i
    println!();

    /* 单元类型 () */
    // 例如main()函数的返回值就是单元类型Result<(), usize>. println!()的返回值也是单元类型()
    // HashSet的val也是()
    let unit_empty = ();
    println!(
        "size of () is {} bytes.",
        std::mem::size_of_val(&unit_empty)
    ); // () is 0 bytes
    println!("{}", std::any::type_name::<()>()); // ()
    let unit = implicitly_ret_unit();
    println!("{:?}", unit); // ()
    assert_eq!(unit, ());

    /* 运算符对应的trait */
    //     <、<=、>、>=    std::cmp::PartialOrd
    //     ==、!=         std::cmp::PartialEq
    //     +              std::ops::Add
    //     -              std::ops::Sub
    //     *              std::ops::Mul
    //     /              std::ops::Div
    //     %(取模)         std::ops::Rem
    //     -(取负)         std::ops::Neg
    //     +=             std::ops::AddAssign
    //     -=             std::ops::SubAssign
    //     *=             std::ops::MulAssign
    //     /=             std::ops::DivAssign
    //     %=             std::ops::RemAssign
}

/// 隐式地返回单元类型
fn implicitly_ret_unit() {
    println!("I will return a ()");
}

/// 显式地返回单元类型
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
    ()
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
