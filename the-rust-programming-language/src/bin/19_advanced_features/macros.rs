#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(deprecated)]

use std::fs::File;
use std::io;
use std::io::{Error, Write};

///
/// 以下内容均为补充
///
/// cargo r --bin std_macros
///
fn main() {
    println!("\n\nhello, macro!\n\n");
    //panic();
    //assert();
    //assert_eq();
    //assert_ne();
    //debug_assert();
    //debug_assert_eq();
    //debug_assert_ne();
    //matches();
    //r#try();
    //write();
    //writeln();
    //unreachable();
    //unimplemented();
    //todo();
    //format_args();
    //format();
    //env();
    //option_env();
    //concat_idents();
    //concat_bytes();
    //concat();
    //line();
    //column();
    //file();
    //stringify();
    //include_str();
    //include_bytes();
    //module_path();
    //cfg();
    //include();
    //log_syntax();
    //trace_macros();
    //vec();
    //print();
    //println();
    //eprint();
    //eprintln();
    //dbg();
    //thread_local();
    //is_x86_feature_detected();
}

fn panic() {
    panic!("i am panicked, game over!");
}

/// Asserts that a boolean expression is true at runtime.
fn assert() {
    assert!(true);

    fn some_computation() -> bool {
        true
    } // a very simple function

    assert!(some_computation());

    // assert with a custom message
    let x = true;
    assert!(x, "x wasn't true!");

    let a = 4;
    let b = 27;
    assert!(a + b == 30, "a = {}, b = {}", a, b);
}

/// Asserts that two expressions are equal to each other (using PartialEq).
fn assert_eq() {
    let a = 3;
    let b = 1 + 3;
    assert_eq!(a, b);
    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
}

/// Asserts that two expressions are not equal to each other (using PartialEq).
fn assert_ne() {
    let a = 3;
    let b = 2;
    assert_ne!(a, b);
    assert_ne!(a, b, "we are testing that the values are not equal");
}

/// Asserts that a boolean expression is true at runtime.
fn debug_assert() {
    let x = true;
    debug_assert!(x, "x wasn't true!");

    let a = 3;
    let b = 27;
    debug_assert!(a + b == 31, "a = {}, b = {}", a, b);
}

/// Asserts that two expressions are equal to each other.
fn debug_assert_eq() {
    let a = 3;
    let b = 1 + 3;
    debug_assert_eq!(a, b, "a = {}, b = {}", a, b);
}

/// Asserts that two expressions are not equal to each other.
fn debug_assert_ne() {
    let a = 3;
    let b = 2;
    debug_assert_ne!(a, b);
}

/// Returns whether the given expression matches any of the given patterns.
fn matches() {
    let foo = 'f';
    let result = matches!(foo, 'A'..='Z' | 'a'..='z');
    assert!(result);

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}

enum MyError {
    FileWriteError,
}
impl From<Error> for MyError {
    fn from(_e: Error) -> MyError {
        MyError::FileWriteError
    }
}
// The preferred method of quick returning Errors
fn write_to_file_question() -> Result<(), MyError> {
    let mut file = File::create("my_best_friends.txt")?;
    file.write_all(b"This is a list of my best friends.")?;
    Ok(())
}

// The previous method of quick returning Errors
fn write_to_file_using_try() -> Result<(), MyError> {
    let mut file = r#try!(File::create("my_best_friends.txt"));
    r#try!(file.write_all(b"This is a list of my best friends."));
    Ok(())
}
/// Unwraps a result or propagates its error.
///
/// try 是保留的关键字，所以只能写成r#try这种形式了
fn r#try() {
    println!("r#try");
}

/// Writes formatted data into a buffer.
fn write() {
    let mut w = Vec::new();
    write!(&mut w, "test ").expect("Failed to write test");
    write!(&mut w, "formatted {}", "arguments").expect("Failed to write...");

    let string = String::from_utf8(w).expect("Invalid UTF8 char");
    println!("{}", string); // test formatted arguments
}

/// Write formatted data into a buffer, with a newline appended.
fn writeln() {
    let mut w = Vec::new();
    writeln!(&mut w).unwrap();
    writeln!(&mut w, "test").unwrap();
    writeln!(&mut w, "formatted {}", "arguments").unwrap();

    assert_eq!(&w[..], "\ntest\nformatted arguments\n".as_bytes());
}

fn foo(x: Option<i32>) {
    match x {
        Some(n) if n >= 0 => println!("Some(Non-negative)"),
        Some(n) if n < 0 => println!("Some(Negative)"),
        Some(_) => unreachable!(), // compile error if commented out
        None => println!("None"),
    }
}
/// Indicates unreachable code.
fn unreachable() {
    foo(Some(5));
}

trait Validate {
    fn validate_order_by_id(&self, order_id: u64) -> Result<bool, Error>;
}

struct Order {}

impl Validate for Order {
    fn validate_order_by_id(&self, order_id: u64) -> Result<bool, Error> {
        unimplemented!("Sorry, No time to impl");
    }
}

/// Indicates unimplemented code by panicking with a message of "not implemented".
fn unimplemented() {
    let order = Order {};
    let _ = order
        .validate_order_by_id(12_u64)
        .expect("Failed to validate");
}

/// Indicates unfinished code.
fn todo() {
    // let's not worry about implementing baz() for now
    todo!();
}

/// Causes compilation to fail with the given error message when encountered.
fn compile_error() {
    // 在自定义声明式宏时使用
    macro_rules! give_me_foo_or_bar {
        (foo) => {};
        (bar) => {};
        ($x:ident) => {
            compile_error!("This macro only accepts `foo` or `bar`");
        };
    }

    give_me_foo_or_bar!(foo); // no problem
                              //give_me_foo_or_bar!(neither); // ^ will fail at compile time with message "This macro only accepts `foo` or `bar`"
}

/// Constructs parameters for the other string-formatting macros.
fn format_args() {
    //let arguments: Arguments = format_args!("hello {}", "world");

    let hello = "hello";
    let world = "world";
    let mut string = std::fmt::format(format_args!("{} {}", hello, world));
    println!("{}", string); // hello world

    // 等价于
    string = format!("{} {}", hello, world);
    println!("{}", string); // hello world
}

/// Creates a String using interpolation of runtime expressions.
fn format() {
    let string = format!("{} {}", "hello", "rust");
    println!("{}", string); // hello rust
}

/// Inspects an environment variable at compile time.
fn env() {
    let path: &'static str = env!("PATH");
    println!("path: {}", path);
    // customize the error message
    //let doc: &'static str = env!("documentation", "what's that?!");
    //println!("doc: {}", doc);
}

/// Optionally inspects an environment variable at compile time.
fn option_env() {
    let key: Option<&'static str> = option_env!("SECRET_KEY");
    println!("the secret key might be: {key:?}");
}

/// Concatenates identifiers into one identifier.
fn concat_idents() {
    // unstable

    /*fn foobar() -> u32 {
        23
    }
    let f = concat_idents!(foo, bar);
    println!("{}", f());*/
}

/// Concatenates literals into a byte slice.
fn concat_bytes() {
    // unstable
    /*
    let s: &[u8; 6] = concat_bytes!(b'A', b"BC", [68, b'E', 70]);
    assert_eq!(s, b"ABCDEF");
    */
}

/// Concatenates literals into a static string slice.
fn concat() {
    let s = concat!("test", 10, 'b', true);
    assert_eq!(s, "test10btrue");
}

/// Expands to the line number on which it was invoked.
fn line() {
    let current_line = line!();
    println!("defined on line: {current_line}"); // 265
}

/// Expands to the column number at which it was invoked.
fn column() {
    let current_col = column!();
    println!("defined on column: {current_col}"); // 23
}

/// Expands to the file name in which it was invoked.
fn file() {
    let this_file = file!();
    println!("defined in file: {this_file}"); // src/bin/x_macros/macros.rs
}

/// Stringifies its arguments.
fn stringify() {
    let one_plus_one = stringify!(1 + 1);
    println!("stringify: {}", one_plus_one); // 1 + 1

    let other1 = stringify!(1 + 1 4 + 6);
    println!("stringify: {}", other1); // 1 + 1 4 + 6

    let other2 = stringify!(1 + 1, 4 + 6);
    println!("stringify: {}", other2); // 1 + 1, 4 + 6
}

/// Includes a UTF-8 encoded file as a string at compile-time,
/// the file content is stored as part of the application binary
///
/// The file is located relative to the current file (similarly to how modules are found).
fn include_str() {
    let my_str = include_str!("../../../hello.html");
    println!("{}", my_str);
}

/// Includes a file as a reference to a byte array at compile-time.
///
/// The file is located relative to the current file (similarly to how modules are found).
fn include_bytes() {
    let my_str = include_bytes!("../../../hello.html");
    println!("{}", String::from_utf8_lossy(my_str));
}

/// Parses a file as an expression or an item according to the context.
fn include() {
    let my_string = include!("monkeys.in");
    assert_eq!("🙈🙊🙉🙈🙊🙉", my_string);
    println!("{my_string}");
}

/// Expands to a string that represents the current module path.
fn module_path() {
    let module_path = module_path!();
    println!("{}", module_path); // std_macros
}

/// Evaluates boolean combinations of configuration flags at compile-time.
fn cfg() {
    /*
    let my_directory = if cfg!(windows) {
        "windows-specific-directory"
    } else {
        "unix-directory"
    };*/
}

/// Prints passed tokens into the standard output.
fn log_syntax() {
    // unstable
}

/// Enables or disables tracing functionality used for debugging other macros.
fn trace_macros() {
    // unstable
}

/// Creates a Vec containing the arguments.
fn vec() {
    let _v1 = vec![1, 2, 3];
    let _v2 = vec![1; 3];
}

/// Prints to the standard output.
/// Note that stdout is frequently line-buffered by default,
/// so it may be necessary to use io::stdout().flush() to ensure the output is emitted immediately.
fn print() {
    print!("this ");
    print!("will ");
    print!("be ");
    print!("on ");
    print!("the ");
    print!("same ");
    print!("line ");

    io::stdout().flush().unwrap();
}

/// Prints to the standard output, with a newline.
fn println() {
    println!(); // prints just a newline
    println!("hello there!");
    println!("format {} arguments", "some");
    let local_variable = "some";
    println!("format {local_variable} arguments");
}

/// Prints to the standard error.
fn eprint() {
    eprint!("Error: Could not complete task");
}

/// Prints to the standard error, with a newline.
fn eprintln() {
    eprintln!("Error: Could not complete task");
}

/// Prints and returns the value of a given expression for quick and dirty debugging.
fn dbg() {
    let a = 2;
    let i = dbg!(a * 2);
    let b = i + 1;
    println!("{}", b);
    // [src/bin/x_macros/macros.rs:410] a * 2 = 4
    // 5
}

/// A thread local storage key which owns its contents.
fn thread_local() {
    thread_local! {
        // 定义线程本地变量，类似于Java中的ThreadLocal
    }
}

/// A macro to test at runtime whether a CPU feature is available on x86/x86-64 platforms.
fn is_x86_feature_detected() {
    let is_support = is_x86_feature_detected!("aes");
    println!("{}", is_support);
}
