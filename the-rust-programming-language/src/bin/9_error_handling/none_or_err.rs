#![allow(unused)]

/// 以下内容皆为补充
///
/// cargo r --bin none_or_err
///
/// ## 目录
/// ### 组合器
/// - 组合器更多的是用于对返回结果的类型进行变换
///
/// ### 归一化不同的错误类型
/// - 使用特征对象 Box<dyn std::error::Error>
/// - 自定义错误类型
/// - 使用三方库  thiserror、anyhow
///
fn main() {
    /* 组合器 */
    // or() 按照从左到右的顺序求表达式的值，若任何一个表达式的结果是 Some/Ok，则该值会立刻返回
    // and() 若两个表达式的结果都是 Some/Ok，则第二个表达式中的值被返回。若任何一个的结果是 None/Err ，则立刻返回None/Err
    // xor() 只能用于Option
    or_and_xor();

    // or_else() 和 and_then() 类似于or/and，只不过第2个表达式是闭包
    or_else_and_then();

    //filter()  仅用于对Option进行过滤
    filter();

    //map() 和 map_err()
    map(); //map可以将Some或Ok中的值映射为另一个Some或Ok
    map_err(); //map_err可以将Err中的值映射为另一个Err

    //map_or() 和 map_or_else()
    map_or(); //在map的基础上提供了一个默认值
    map_or_else(); //与map_or类似，但是它是通过一个闭包来提供默认值

    //ok_or() 和 ok_or_else()  可以将Option类型转换为Result类型
    ok_or(); // Option -> Result
    ok_or_else(); // 同上，接收一个闭包作为Err参数

    /* 归一化不同的错误类型 */
    // 1、std::error::Error
    //
    // pub trait Error: Debug + Display {
    //     // 这个方法提供了默认实现，自定义实现时可以不用处理，实现Debug + Display即可
    //     fn source(&self) -> Option<&(dyn Error + 'static)> {
    //         None
    //     }
    // }
    //

    // 2、自定义错误类型
    //AppError { kind: "io", msg: "系统找不到指定的文件。 (os error 2)" }
    let _ = custom_error().map_err(|e| println!("{:?}", e));
    println!();

    // 3、thiserror
    //IOError(Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" })
    let _ = this_error().map_err(|e| println!("{:?}", e));
    println!();

    // 3、anyhow
    // Use Anyhow if you don't care what error type your functions return, you just want it to be easy.
    let _ = any_how().map_err(|e| println!("{:?}", e)); // invalid digit found in string
}

fn or_and_xor() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;
    assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
    assert_eq!(s1.or(n), s1); // Some or None = Some
    assert_eq!(n.or(s1), s1); // None or Some = Some
    assert_eq!(n.or(n), n); // None1 or None2 = None2/None1
    assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
    assert_eq!(s1.and(n), n); // Some and None = None
    assert_eq!(n.and(s1), n); // None and Some = None
    assert_eq!(n.and(n), n); // None1 and None2 = None1
    assert_eq!(s1.xor(s2), n); // Some1 xor Some2 = None
    assert_eq!(s1.xor(n), s1); // Some1 xor None = Some1
    assert_eq!(n.xor(s1), s1); // None xor Some1 = Some1
    assert_eq!(n.xor(n), n); // None xor None = None
    assert_eq!(s1.xor(s1), n); // Some1 xor Some1 = None

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
    assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
    assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
    assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2
    assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
    assert_eq!(o1.and(e1), e1); // Ok and Err = Err
    assert_eq!(e1.and(o1), e1); // Err and Ok = Err
    assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1
}
fn or_else_and_then() {
    // or_else with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };
    let fn_some_2 = |_| Some("some2");

    let n: Option<&str> = None;
    let fn_none = || None;
    let fn_none_2: fn(_) -> Option<&'static str> = |_| None;

    assert_eq!(s1.or_else(fn_some), s1); // Some1 or_else Some2 = Some1
    assert_eq!(s1.or_else(fn_none), s1); // Some or_else None = Some
    assert_eq!(n.or_else(fn_some), s2); // None or_else Some = Some
    assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2
    assert_eq!(s1.and_then(fn_some_2), s2); // Some1 and_then Some2 = Some2
    assert_eq!(s1.and_then(fn_none_2), None); // Some and_then None = None
    assert_eq!(n.and_then(fn_some_2), None); // None and_then Some = None
    assert_eq!(n.and_then(fn_none_2), None); // None and_then None = None

    // or_else with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
    assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
    assert_eq!(e1.or_else(fn_ok), o2); // Err or_else Ok = Ok
    assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2
    assert_eq!(o1.and_then(fn_ok), o2); // Ok1 and_then Ok2 = OK2
    assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
    assert_eq!(e1.and_then(fn_ok), e1); // Err and_then Ok = Err
    assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
}
fn filter() {
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;
    let fn_is_even = |x: &i8| x % 2 == 0;
    assert_eq!(s1.filter(fn_is_even), n); // Some(3) -> 3 is not even -> None
    assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
    assert_eq!(n.filter(fn_is_even), n); // None -> no value -> None

    let ok1: Result<i32, &str> = Ok(5);
    let ok2: Result<i32, &str> = Ok(6);
    let err: Result<i32, &str> = Err("parameter wrong");
    //Result 没有直接提供filter方法
}
fn map() {
    let s1 = Some("abcde");
    let s2 = Some(5);
    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);
    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");
    let fn_character_count = |s: &str| s.chars().count();

    assert_eq!(s1.map(fn_character_count), s2); // Some1 map = Some2
    assert_eq!(n1.map(fn_character_count), n2); // None1 map = None2
    assert_eq!(o1.map(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map(fn_character_count), e2); // Err1 map = Err2
}
fn map_err() {
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2
}
fn map_or() {
    const V_DEFAULT: u32 = 1;
    let fn_closure = |v: u32| v + 2;

    let ok: Result<u32, ()> = Ok(10);
    let err: Result<_, u32> = Err(10);
    assert_eq!(ok.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(err.map_or(V_DEFAULT, fn_closure), V_DEFAULT);

    let some: Option<u32> = Some(10);
    let none: Option<u32> = None;
    assert_eq!(some.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(none.map_or(V_DEFAULT, fn_closure), V_DEFAULT);
}
fn map_or_else() {
    let fn_closure = |v: i8| v + 2;
    let fn_default = || 1;

    let some = Some(10);
    let none: Option<i8> = None;
    assert_eq!(some.map_or_else(fn_default, fn_closure), 12);
    assert_eq!(none.map_or_else(fn_default, fn_closure), 1);

    let ok = Ok(10);
    let err = Err(5);
    let fn_default_for_result = |v: i8| v + 1; // 闭包可以对 Err 中的值进行处理，并返回一个新值
    assert_eq!(ok.map_or_else(fn_default_for_result, fn_closure), 12);
    assert_eq!(err.map_or_else(fn_default_for_result, fn_closure), 6);
}
fn ok_or() {
    const ERR_DEFAULT: &str = "error message";

    let some = Some("abcde");
    let none: Option<&str> = None;

    let ok: Result<&str, &str> = Ok("abcde");
    let err: Result<&str, &str> = Err(ERR_DEFAULT);

    assert_eq!(some.ok_or(ERR_DEFAULT), ok); // Some(T) -> Ok(T)
    assert_eq!(none.ok_or(ERR_DEFAULT), err); // None -> Err(default)
}
fn ok_or_else() {
    let some = Some("abcde");
    let none: Option<&str> = None;
    let fn_err_message = || "error message";

    let ok: Result<&str, &str> = Ok("abcde");
    let err: Result<&str, &str> = Err("error message");

    assert_eq!(some.ok_or_else(fn_err_message), ok); // Some(T) -> Ok(T)
    assert_eq!(none.ok_or_else(fn_err_message), err); // None -> Err(default)
}

#[derive(Debug)]
struct CustomAppError {
    kind: String, // 错误类型
    msg: String,  // 错误内容
}
///
/// 为每种系统错误实现From Trait以转化为自定义错误类型
///
impl From<std::io::Error> for CustomAppError {
    fn from(value: std::io::Error) -> Self {
        CustomAppError {
            kind: "io".into(),
            msg: value.to_string(),
        }
    }
}

fn custom_error() -> Result<(), CustomAppError> {
    let string = std::fs::read_to_string("app.txt")?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("Environment variable not found")]
    EnvironmentVariableNotFound(#[from] std::env::VarError),
    #[error("Can't find file")]
    IOError(#[from] std::io::Error),
}

fn this_error() -> Result<(), AppError> {
    let string = std::fs::read_to_string("app.txt")?;
    Ok(())
}

fn any_how() -> anyhow::Result<()> {
    let val: i32 = "12x".parse()?;
    println!("{}", val);
    Ok(())
}
