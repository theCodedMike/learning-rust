use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};

/// 9.2 Result 与可恢复的错误
/// cargo r --bin 9_2
fn main() -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> 被称为 “trait 对象”（trait object），意味着可以返回任何类型的错误
    // 等价于  &dyn Error
    /*
    ## Result 与可恢复的错误
    ### 匹配不同的错误
    ### 失败时 panic 的简写：unwrap 和 expect
    ### 传播错误
    ### 传播错误的简写：? 运算符
    ### ? 运算符可被用于返回 Result 的函数
    ###
     */

    // Result 与可恢复的错误
    let f = File::open("E:\\test code.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };

    // 匹配不同的错误
    let f = File::open("E:\\test code.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("E:\\test code.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 失败时 panic 的简写：unwrap 和 expect
    let f = File::open("E:\\test code.txt").unwrap(); //如果文件不存在直接panic
    let f = File::open("E:\\test code.txt").expect("Failed to open hello.txt"); // 同上，但提供了panic时的错误信息

    // 传播错误
    let result = read_username_from_file();

    // 传播错误的简写：? 运算符
    let _ = read_username_from_file_use_question_mark();
    let _ = read_username_from_file_use_question_mark_2();

    // ? 运算符可被用于返回 Result 的函数
    let f = File::open("E:\\test code.txt")?;
    Ok(())
}
fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("E:\\test code.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn read_username_from_file_use_question_mark() -> Result<String, std::io::Error> {
    let mut f = File::open("E:\\test code.txt")?; // 有错误直接往外抛
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn read_username_from_file_use_question_mark_2() -> Result<String, std::io::Error> {
    let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    fs::read_to_string(&mut s)?;
    Ok(s)
}