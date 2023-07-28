#![allow(dead_code)]
#![allow(unused_variables)]

/// 3.5 控制流
///
/// cargo r --bin 3_5
///
/// ## 目录
///
/// ### 分支
/// #### 使用 else if 处理多个分支
/// #### 在 let 表达式中使用 if
///
/// ### 循环
/// 可以搭配 continue break 使用
/// #### 使用 loop
/// #### 在 loop 中返回值
/// #### while
/// #### for
///
fn main() {
    /* if表达式 */
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // rust版的3目运算符
    println!("The value of number is: {}", number);
    println!();

    /* loop循环 */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result); // 20
    println!();

    /* while循环 */
    let mut number = 3;
    while number != 0 {
        print!("{} ", number);
        number -= 1;
    }
    println!("\n");

    /* for循环 */
    let a = [10, 20, 30, 40, 50];
    for element in a {
        // 注意这里a失去了其所有权
        print!("{} ", element); // 10 20 30 40 50
    }
    println!();
    let a = [10, 20, 30, 40, 50];
    for element in &a {
        // 这样a就不会失去其所有权
        print!("{} ", element); // 10 20 30 40 50
    }
    println!();
    //   使用方法                         等价使用方式                                         所有权
    // for item in collection         for item in IntoIterator::into_iter(collection)    转移所有权(如果集合中不是基本类型)
    // for item in &collection        for item in collection.iter()                      不可变引用
    // for item in &mut collection    for item in collection.iter_mut()                  可变引用
    for (idx, val) in a.iter().enumerate() {
        if idx == 1 {
            continue;
        }
        println!("{idx}: {val}");
        // 0: 10
        // 2: 30
        // 3: 40
        // 4: 50
    }
    for number in 0..a.len() {
        if number == 10 {
            break;
        }
        print!("{} ", number); // 0 1 2 3 4
    }
    println!();
    for number in 1..=4 {
        print!("{} ", number); // 1 2 3 4
    }
    println!();
    for number in (1..4).rev() {
        print!("{} ", number); // 3 2 1
    }
    println!();
}
