#![allow(dead_code)]
#![allow(unused_variables)]

/// 8.2 使用String存储UTF-8编码的文本
/// cargo r --bin 8_2
fn main() {
    /*
    ## 使用字符串存储 UTF-8 编码的文本
    ### 什么是字符串？
    - Rust 的核心语言中只有一种字符串类型：str，即字符串切片；它通常以被借用的形式出现，&str，即字符串切片引用
    - 称作 String 的类型是由标准库提供的
    ## 新建字符串
    ## 更新字符串
    ### 使用 push_str 和 push 附加字符串
    - 解引用强制转换（deref coercion）: &String 可以被 强转（coerced）成 &str
    ### 使用 + 运算符或 format! 宏拼接字符串
    ## 索引字符串
    ## 字符串 slice
    ## 遍历字符串的方法
    - Unicode 标量值可能会由不止一个字节组成

     */
    // 新建字符串
    let s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // 该方法也可直接用于字符串字面量：
    let mut s = "initial contents".to_string();
    let mut s2 = String::from(data);
    println!("here s is '{}', s2 is '{}'", s, s2);
    s.push_str(", rust");
    s2.insert_str(0, "e890w ");
    println!(" s is '{}'", s); // initial contents, rust
    println!("s2 is '{}'", s2); // e890w initial contents

    // 更新字符串 push_str/push/+/format!宏
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s); // foobar

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s); // lol

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
                       //println!("s1 is {}", s1); // 本质上相当于调用了add方法
                       /*impl Add<&str> for String {
                           fn add(mut self, other: &str) -> String {
                               self.push_str(other);
                               self
                           }
                       }*/
    println!("s2 is {}", s2); // world!
    println!("s3 is {}", s3); // Hello, world!
                              // 对于多个字符串，+ 就显得笨重了，建议使用format!宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let multi_plus = s1 + "-" + &s2 + "-" + &s3;
    println!("result is {}", multi_plus); // tic-tac-toe
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let format = format!("{}-{}-{}", s1, s2, s3);
    println!("result is {}", format); // tic-tac-toe

    // 索引字符串
    let s1 = String::from("hello");
    //let h = s1[0];  //这种索引方式是错误的
    //let h = &s1[0]; //这种索引方式也是错误的
    let char_arr: Vec<u8> = vec![
        224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    ];
    let str = String::from_utf8(char_arr).expect("wrong u8 chars");
    println!("str is {}, len is {}", str, str.len()); // str is नमस्ते, len is 18
                                                      // 由上可见，不能使用索引来使用字符串

    // 字符串 slice
    let hello = "Здравствуйте".to_string();
    let s = &hello[0..4];
    println!("string is {}", s); // string is Зд
                                 //let s2 = &hello[0..1]; // panic, 因为第1个字节不表示1个字符
    let s2 = &hello[0..2];
    println!("here char is {}", s2); // char is 3

    // 遍历字符串的方法
    for char in hello.chars() {
        println!("char is {}", char);
    }
    /*
    char is З
    char is д
    char is р
    char is а
    char is в
    char is с
    char is т
    char is в
    char is у
    char is й
    char is т
    char is е
    */

    let chars = b"hello";
    println!("{:?}", chars); // [104, 101, 108, 108, 111]
    let row_chars = r"hello\nworld";
    println!("{}", row_chars); // hello\nworld  这里并没有换行，因为r表示原始的，\n就是普通的2个字符
    let chars = "hello\nworld";
    println!("{}", chars);
    // hello  这里有换行
    // world
}
