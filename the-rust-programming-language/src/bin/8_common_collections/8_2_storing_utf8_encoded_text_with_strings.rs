#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Add;

/// 8.2 使用String存储UTF-8编码的文本
///
/// cargo r --bin string
///
/// ## 目录
/// ### 使用字符串存储 UTF-8 编码的文本
/// #### 什么是字符串？
/// - Rust 的核心语言中只有一种字符串类型: str，即字符串切片；它通常以被借用的形式出现，&str，即字符串切片引用
/// - 称作 String 的类型是由标准库提供的
///
/// ### 新建字符串
///
/// ### 更新字符串
/// #### 使用 push_str 和 push 附加字符串
/// - 解引用强制转换（deref coercion）: &String 可以被 强转（coerced）成 &str
/// #### 使用 + 运算符或 format! 宏拼接字符串
///
/// ### 索引字符串
///
/// ### 字符串 slice
///
/// ### 遍历字符串的方法
/// - Unicode 标量值可能会由不止一个字节组成
///
fn main() {
    /* 新建字符串 */
    let s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // 该方法也可直接用于字符串字面量：
    let mut s = "initial contents".to_string();
    let mut s2 = String::from(data);
    println!("here s is '{}', s2 is '{}'", s, s2); // here s is 'initial contents', s2 is 'initial contents'
    s.push_str(", rust");
    s2.insert_str(0, "e890w ");
    println!(" s is '{}'", s); // initial contents, rust
    println!("s2 is '{}'", s2); // e890w initial contents
    println!();

    /* 更新字符串 push_str/push/+/format!宏 */
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s); // foobar

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s); // lol

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // 注意 s1 被移动了，不能继续使用
    //println!("s1 is {}", s1); // 本质上相当于调用了add方法
    /*
    impl Add<&str> for String {
        fn add(mut self, other: &str) -> String {
            self.push_str(other);
            self
        }
    }
    */
    println!("s2 is {}", s2); // world!
    println!("s3 is {}", s3); // Hello, world!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let multi_plus = s1 + "-" + &s2 + "-" + &s3;
    println!("result is {}", multi_plus); // tic-tac-toe
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 对于多个字符串，+ 就显得笨重了，建议使用format!宏
    let format = format!("{}-{}-{}", s1, s2, s3);
    println!("result is {}", format); // tic-tac-toe
    println!();

    /* 索引字符串 */
    let s1 = String::from("hello");
    //let h = s1[0];  //这种索引方式是错误的
    //let h = &s1[0]; //这种索引方式也是错误的
    let char_arr: Vec<u8> = vec![
        224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    ];
    let str = String::from_utf8(char_arr).expect("wrong u8 chars");
    // 不能使用索引来访问字符串，是因为字符串中的字符是不定长的，索引不一定刚好位于字符的起始位置
    println!("str is {}, len is {}", str, str.len()); // str is नमस्ते, len is 18

    /* 字符串 slice */
    let hello = "Здравствуйте".to_string();
    let s = &hello[0..4];
    println!("string is {}", s); // string is Зд
                                 //let s2 = &hello[0..1]; // panic, 因为第1个字节不表示1个字符
    let s2 = &hello[0..2];
    println!("here char is {}", s2); // char is 3
    println!();

    /* 遍历字符串的方法 */
    for char in hello.chars() {
        print!("{} ", char); // З д р а в с т в у й т е
    }
    println!();
    let chars = b"hello";
    println!("{:?}", chars); // [104, 101, 108, 108, 111]
    let row_chars = r"hello\nworld";
    println!("{}", row_chars); // hello\nworld  这里并没有换行，因为r表示原始的，\n就是普通的2个字符
    let chars = "hello\nworld";
    println!("{}", chars);
    // hello  这里有换行
    // world
    println!();

    /* 字符串操作 补充 */
    // 操作字符串-插入 只适用于String
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("    insert(): {}", s); // Hello, rust!
    s.insert_str(6, " I like");
    println!("insert_str(): {}", s); // Hello, I like rust!
    println!();

    // 操作字符串-替换 适用于String、&str  会生成新的字符串，而不是更新原来的
    let before = String::from("I like rust. Learning rust is my favorite!");
    let after = before.replace("rust", "RUSTxx");
    println!("after replace(): {}", after); // I like RUSTxx. Learning RUSTxx is my favorite!

    let str_ref = "haha, today i'm so happy!";
    let after = str_ref.replace("ha", "HA");
    println!("after replace(): {}", after); // HAHA, today i'm so HAppy!

    let after = before.replacen("rust", "RUSTxx", 1);
    println!("after replacen(): {}", after); // I like RUSTxx. Learning rust is my favorite!

    let mut str_replace_range = String::from("I like rust!");
    str_replace_range.replace_range(7..8, "R"); //只适用于String类型
    println!("after replace_range(): {}", str_replace_range); // I like Rust!
    println!();

    // 操作字符串-删除 只适用于String
    let mut s = String::from("rust pop 中文!");
    let val_from_pop = s.pop(); //删除并返回字符串的最后一个字符
    assert_eq!(val_from_pop, Some('!'));
    println!("after pop(): {}", s); // rust pop 中文
    println!("val of pop: {}", val_from_pop.unwrap()); // !

    let mut s = String::from("测试remove方法");
    //删除并返回字符串中指定位置的字符。该方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会panic
    let removed_char = s.remove(3);
    println!("after remove(): {}", s); // 测remove方法
    println!("val of remove: {}", removed_char); // 试

    let mut s = String::from("测试truncate");
    //删除字符串中从指定位置开始到结尾的全部字符。该方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会panic
    s.truncate(3);
    println!("after truncate(): {}", s); // 测

    let mut s = String::from("string clear");
    s.clear(); //清空字符串
    println!(
        "after clear(): {}, and len is {}, and is empty? {}",
        s,
        s.len(),
        s.is_empty()
    ); // 0, true
    println!();

    // 操作字符串-拼接
    let str_append = String::from("hello ");
    let str_rust = String::from("rust");
    let mut result = str_append + &str_rust; //相当于调用了std::string标准库中的add()方法, +右侧必须传递&str类型
    result += "!";
    result = result.add(" I love it!");
    println!("after +/add(): {}", result); // hello rust! I love it!
                                           // 无法通过编译，str_append的所有权已失效，其所有权已移交到add()方法里了
                                           // println!("str_append = {}", str_append);
    println!("str_rust = {}", str_rust); // rust 可以通过编译

    let s1 = "hello".to_string();
    let s2 = "rust";
    let s = format!("{} {}!", s1, s2); //format!适用String类型和&str类型
    println!("after format!: s = {}", s); // hello rust!
    println!("after format!: s1 = {}", s1); // hello
    println!("after format!: s2 = {}\n", s2); // rust
    println!();

    // 字符串转义
    //通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape); // What are you doing? (\x3F means ?) I'm writing Rust!
    println!();

    //\u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    ); // Unicode character ℝ (U+211D) is called "DOUBLE-STRUCK CAPITAL R"
    println!();

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals \
                        can span multiple lines.\
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("单行显示:\n{}", long_string);
    // String literals can span multiple lines.The linebreak and indentation here -><- can be escaped too!
    println!();
    let long_string = "String literals\n\
        can span multiple lines.\n\
        The linebreak and indentation here ->\n\
        <- can be escaped too!";
    println!("多行显示:\n{}", long_string);
    println!();

    //保持原始，不需要转义
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str); // Escapes don't work here: \x3F \u{211D}
                             // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes); // And then I said: "There is no escape!"
                            // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter); // A string with "# in it. And even "##!
    println!();

    // 从字符串中获取单个字符
    let hanzi = "中国人";
    println!(
        "size of \"{hanzi}\" is {} bytes.",
        std::mem::size_of_val(hanzi)
    ); // 9 bytes
    let all_chars = hanzi.chars();
    println!("count of hanzi.chars() is {}", all_chars.clone().count()); // 3
    for a_char in all_chars {
        print!("{} ", a_char); // 中 国 人
    }
}
