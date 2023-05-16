use std::slice;

/// 19.1 不安全Rust
///
/// cargo r --bin 19_1
///
/// ## 目录:
/// ### 不安全的超能力
/// unsafe 意图在于作为开发者你将会确保 unsafe 块中的代码以有效的方式访问内存
/// - 解引用裸指针
/// - 调用不安全的函数或方法
/// - 访问或修改可变静态变量
/// - 实现不安全 trait
/// - 访问 union 的字段
///
/// ### 解引用裸指针
/// 裸指针与引用和智能指针的区别在于：
/// - 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
/// - 不保证指向有效的内存
/// - 允许为空
/// - 不能实现任何自动清理功能
///
/// ### 调用不安全函数或方法
/// #### 创建不安全代码的安全抽象
/// #### 使用 extern 函数调用外部代码
/// ### 访问或修改可变静态变量
/// - 静态变量可以是可变的，也可以是不变的
/// - 静态变量只能储存拥有 'static 生命周期的引用
/// - 静态变量中的值有一个固定的内存地址，而常量则允许在任何被用到的时候复制其数据
///
/// ### 实现不安全 trait
/// ### 访问 union 的字段
///

static TITLE: &str = "Hello, world!";
static mut COUNTER: u32 = 0;
fn main() {
    // 解引用裸指针
    let mut num = 5;
    let r1 = &num as *const i32; //创建裸指针是安全的
    let r2 = &mut num as *mut i32;
    println!("{:p}", r1); //0x51023ff7b4
    println!("{:p}", r2); //0x51023ff7b4
    unsafe {
        println!("r1 is {}", *r1); //5
        println!("r1 is {}", *r2); //5
    }

    // 调用不安全函数或方法
    unsafe {
        dangerous();
    }

    // 创建不安全代码的安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    println!("first segment: {:?}", Vec::from(a)); //[1, 2, 3]
    println!("first segment: {:?}", Vec::from(b)); //[4, 5, 6]

    // 使用 extern 函数调用外部代码
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 访问或修改可变静态变量
    add_to_count(5);
    unsafe {
        println!("now COUNTER is {}", COUNTER);
    }
    println!("now TITLE is {}", TITLE); //不用被unsafe包围

    // 实现不安全 trait
    let my_struct = MyStruct::new(45, "light");
    my_struct.print();

    // 访问联合体中的字段
    let mut u = IntOrFloat { f: 1.0 };
    println!("now i is {}", unsafe { u.i }); // 1065353216
    println!("now f is {}", unsafe { u.f }); // 1
}
unsafe fn dangerous() {
    println!("this is a unsafe function");
}
/// cannot borrow `*slice` as mutable more than once at a time
/// 此时用引用就无法走通，必须用指针
/*fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
}*/
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
extern "C" {
    fn abs(input: i32) -> i32;
}
// 从 C 代码中调用 Rust 接口
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
unsafe trait Foo {
    // methods go here
    fn print(&self);
}
struct MyStruct {
    count: usize,
    name: String,
}
unsafe impl Foo for MyStruct {
    // method implementations go here
    fn print(&self) {
        println!("MyStruct: count = {}, name = {}", self.count, self.name);
    }
}
impl MyStruct {
    fn new(count: usize, name: &str) -> Self {
        MyStruct {
            count,
            name: String::from(name),
        }
    }
}
union IntOrFloat {
    i: u32,
    f: f32,
}
