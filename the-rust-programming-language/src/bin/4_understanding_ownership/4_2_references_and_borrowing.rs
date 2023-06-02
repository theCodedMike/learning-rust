#![allow(dead_code)]
#![allow(unused_variables)]

/// 4.2 引用和借用
/// cargo r --bin 4_2
fn main() {
    /*
    ## 引用与借用
    - 与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符 *
    - 我们将创建一个引用的行为称为 借用（borrowing）

    ## 可变引用
    - 在同一时间，只能有一个对某一特定数据的可变引用
    ### 数据竞争（data race）
    - 两个或更多指针同时访问同一数据
    - 至少有一个指针被用来写入数据
    - 没有同步数据访问的机制

    ## 悬垂引用（Dangling References）
    其指向的内存可能已经被分配给其它持有者

    ## 引用的规则
    - 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用
    - 引用必须总是有效的
     */
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 不可变引用
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("The length of '{}' is {}.", s, s.len());

    let mut s = String::from("hello");
    let r1 = &mut s; // cannot borrow `s` as mutable more than once at a time
                     //let r2 = &mut s; // second mutable borrow occurs here
                     //println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    println!("len is {}", r1.len());
    let r2 = &s; // 没问题
    println!("len 2 is {}", r2.len());
    let r3 = &mut s; // 大问题  cannot borrow `s` as mutable because it is also borrowed as immutable
    r3.push_str(", world");
    println!("str is {}", r3);
    //println!("{}, {}, and {}", r1, r2, r3); // 编译报错

    // cannot return reference to local variable `s`
    //let reference_to_nothing = dangle();
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/*fn dangle<'a>() -> &'a String {
    let s = String::from("hello");

    &s // returns a reference to data owned by the current function
}*/
