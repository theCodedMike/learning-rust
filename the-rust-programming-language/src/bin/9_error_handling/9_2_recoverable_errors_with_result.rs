#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(deprecated)]

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};

const FILE_PATH: &str = "E:\\test code.txt";

/// 9.2 用 Result 处理可恢复的错误
///
/// cargo r --bin result
///
/// ## 目录
/// ### 匹配不同的错误
///
/// ### 失败时 panic 的简写：unwrap 和 expect
///
/// ### 传播错误
///
/// ### 传播错误的简写: ? 运算符
///
/// ### ? 运算符可被用于返回 Result 的函数
///
fn main() -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> 被称为 "trait 对象" (trait object), 意味着可以返回任何类型的错误
    // 等价于  &dyn Error
    let f = File::open(FILE_PATH);
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    /* 匹配不同的错误 */
    let f = File::open(FILE_PATH);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_PATH) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    /* 失败时 panic 的简写：unwrap 和 expect */
    let f = File::open(FILE_PATH).unwrap(); //如果文件不存在直接panic
    let f = File::open(FILE_PATH).expect("Failed to open hello.txt"); // 同上，但提供了panic时的错误信息

    /* 传播错误 */
    let result = read_username_from_file();

    /* 传播错误的简写：? 运算符 */
    let _ = read_username_from_file_use_question_mark();
    let _ = read_username_from_file_use_question_mark_2();

    /* ? 运算符可被用于返回 Result/Option 的函数 */
    // the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
    let f = File::open(FILE_PATH)?;
    let even = wrapper(10).ok_or("error")?; // 这里需要把option转换为result, 再使用?

    /* try! 在?横空出世之前(Rust 1.13), Rust开发者还可以使用try!来处理错误, 目前try!已被废弃 */
    let re = r#try!(read_username_from_file());

    Ok(())
}
fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open(FILE_PATH);

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
    let mut f = File::open(FILE_PATH)?; // 有错误直接往外抛
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    //等价于 File::open(FILE_PATH)?.read_to_string(&mut s)?

    Ok(s)
}
fn read_username_from_file_use_question_mark_2() -> Result<String, std::io::Error> {
    let s = fs::read_to_string(FILE_PATH)?;

    Ok(s)
}

fn wrapper(val: i32) -> Option<i32> {
    let i = use_question_mark_for_option(val)?;
    println!("{}", i);
    Some(i)
}

fn use_question_mark_for_option(val: i32) -> Option<i32> {
    if val % 2 == 0 {
        Some(val)
    } else {
        None
    }
}
