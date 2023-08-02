#![allow(dead_code)]
#![allow(unused_variables)]

/// 8.1 使用vector存储一系列值
///
/// cargo r --bin vec
///
/// ## 目录
/// ### 新建 vector
///   可以当作stack使用
///   详细的方法见同目录下的Vec.md
///
/// ### 更新 vector
///
/// ### 丢弃 vector 时也会丢弃其所有元素
///
/// ### 读取 vector 的元素
///
/// ### 遍历 vector 中的元素
///
/// ### 使用枚举来储存多种类型
///
fn main() {
    /* 新建 vector */
    let v: Vec<i32> = Vec::new();
    let v: Vec<usize> = Vec::with_capacity(5);
    let v = vec![1, 2, 3];

    /* 更新 vector */
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v); // [5, 6, 7, 8]
    let removed_val = v.remove(0);
    println!("{}, {:?}", removed_val, v); // 5, [6, 7, 8]

    // 丢弃 vector 时也会丢弃其所有元素
    {
        let v_local = vec![1, 2, 3, 4];

        // 处理变量 v
    } // <- 这里 v_local 离开作用域并被丢弃
      // println!("{:?}", v_local);

    /* 读取 vector 的元素 */
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // 索引
    println!("The third element is {}", third); // 3
    match v.get(2) {
        // get方法
        Some(third) => println!("The third element is {}", third), // 3
        None => println!("There is no third element."),
    }
    // let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100); // 不会panic
    println!("{:?}", does_not_exist); // None
    println!();

    /* 遍历 vector 中的元素 */
    let v = vec![100, 32, 57];
    for i in &v {
        // 等价于使用v.iter()
        print!("{} ", i); // 100 32 57
    }
    println!();
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // 等价于使用v.iter_mut()
        *i += 50;
    }
    println!("{:?}", v); // [150, 82, 107]
    println!();

    /* 使用枚举来储存多种类型 */
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    /* 补充: vec扩容规律 */
    test_vec_cap_allocate();
}
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

/// Vec的动态内存扩容规律
///
/// 4 -> 8 -> 16 -> 32 -> 64 -> 128...
fn test_vec_cap_allocate() {
    let mut vec = Vec::new();
    for i in 1..=150 {
        vec.push(i);
        println!(
            "{:3} inserted, and vec size: {:3}, cap: {:3}",
            i,
            vec.len(),
            vec.capacity()
        );
    }
}
