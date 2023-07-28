#![allow(dead_code)]
#![allow(unused_variables)]

/// 4.3 切片slice
///
/// cargo r --bin 4_3
///
/// ## 目录
/// slice允许引用集合中一段连续的元素序列，而不用引用整个集合
///
/// ## 字符串 slice
/// slice 的数据结构存储了 slice 的开始位置和长度
/// ### 字符串字面量就是 slice
/// ### 字符串 slice 作为参数
///
///   str(无法直接声明并使用)   String
///   &str                   &String
///   &mut str               &mut String
///
///
/// ## 其他类型的 slice (即数组 slice)
///   [T]
///   &[T]
///   &mut [T]
///
fn main() {
    /*
    编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词
    */
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5
    s.clear(); // 这清空了字符串，使其等于 ""
               // word 在此处的值仍然是 5，
               // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
               //println!("now first word is {}", s.index(..word)); // 编译时不报错，运行时panic

    /* 字符串 slice */
    let mut s = String::from("hello world");
    // the size for values of type `str` cannot be known at compilation time
    // let s: str = "hello".into(); // 不能这么声明
    let hello = &s[0..5]; // &s[..5]
    let world = &s[6..11]; // &s[6..]
    let immut_ref = s.as_str(); // 等价于 &s[..]  &s
    let tmp_str = "tmp_str";
    let tmp_str2 = &s;
    let tmp_str3 = &s[..];
    let mut_ref = s.as_mut(); // 等价于 &mut s
    let mut_ref2 = &mut s;
    let first_world = first_word_use_ref(&s);
    //s.clear(); // 编译时 error!
    println!("now first word is {}", first_world); // hello

    let s2 = "hello rust";
    // the type `str` cannot be indexed by `{integer}` 字符串不能使用索引下标来取值
    // println!("string use idx: {}", s2[2]);
    println!();

    /* 字符串 slice 作为参数 */
    let my_string = String::from("hello world");
    // `first_word` 接受 `String` 的切片，无论是部分还是全部
    let word = first_word_use_ref(&my_string[0..6]);
    let word = first_word_use_ref(&my_string[..]);
    // `first_word` 也接受 `String` 的引用，
    // 这等同于 `String` 的全部切片
    let word = first_word_use_ref(&my_string);
    let my_string_literal = "hello world";
    // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    let word = first_word_use_ref(&my_string_literal[0..6]);
    let word = first_word_use_ref(&my_string_literal[..]);
    // 因为字符串字面值**就是**字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word_use_ref(my_string_literal);

    /* 其他类型的 slice */
    let mut a = [1, 2, 3, 4, 5];
    let x = a.as_mut();
    x.fill(6);
    let slice = &a[..];
    println!("slice is {:?}", slice); // [6, 6, 6, 6, 6]
    let dyn_arr = vec![1, 2, 3, 4, 5];
    let a_slice = &dyn_arr[1..3];
    println!("slice is {:?}", a_slice); // [2, 3]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_use_ref(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
