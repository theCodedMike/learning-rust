use std::arch::asm;
use std::slice;

static TITLE: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

/// 19.1 不安全Rust
///
/// cargo r --bin unsafe
///
/// ## 目录
/// ### 不安全的超能力
/// unsafe 意图在于作为开发者你将会确保 unsafe 块中的代码以有效的方式访问内存
/// - 解引用裸指针
/// - 调用不安全的函数或方法
/// - 访问或修改可变静态变量
/// - 实现不安全 trait
/// - 访问 union 的字段
///
/// ### 解引用裸指针(raw pointers)
/// 裸指针与引用和智能指针的区别在于：
/// - 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
/// - 不保证指向有效的内存
/// - 允许为空
/// - 不能实现任何自动清理功能
///
/// ### 调用不安全函数或方法
/// #### 创建不安全代码的安全抽象
/// - 仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的
/// - 将不安全代码封装进安全函数是一个常见的抽象
/// #### 使用 extern 函数调用外部代码
/// - extern有助于创建和使用 外部函数接口(Foreign Function Interface，FFI)
///
/// ### 访问或修改可变静态变量
/// - 静态变量可以是可变的，也可以是不变的
/// - 静态变量只能储存拥有 'static 生命周期的引用
/// - 静态变量中的值有一个固定的内存地址，常量允许在任何被用到的时候复制其数据
/// - 访问不可变静态变量是安全的
///
/// ### 实现不安全 trait
///
/// ### 访问 union 的字段
///
/// ### 何时使用不安全代码
/// - 当有理由需要使用 unsafe 代码时，无需深思熟虑，用之即可
///
fn main() {
    /* 不安全的超能力 */

    /* 解引用裸指针 */
    let mut num = 5;
    let r1 = &num as *const i32; //创建裸指针是安全的
    let r2 = &mut num as *mut i32;
    println!("{:p}", r1); // 0x51023ff7b4
    println!("{:p}", r2); // 0x51023ff7b4
    unsafe {
        println!("num is {}", *r1); // 5
        println!("num is {}", *r2); // 5
    }
    //*r1 = 10; // Cannot assign to immutable dereference of raw pointer
    unsafe {
        *r2 = 20;
        println!("num is {:?}", *r2); // 20
    }
    println!();

    /* 调用不安全函数或方法 */
    unsafe {
        dangerous();
    }
    println!();
    // 创建不安全代码的安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    println!(" first segment: {:?}", Vec::from(a)); // [1, 2, 3]
    println!("second segment: {:?}", Vec::from(b)); // [4, 5, 6]
    println!();
    // 使用 extern 函数调用外部代码
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3)); // 3
    }
    println!();

    /* 访问或修改可变静态变量 */
    add_to_count(5);
    unsafe {
        println!("now COUNTER is {}", COUNTER); // 5
    }
    println!("now TITLE is {}", TITLE); //不用被unsafe包围
    println!();

    /* 实现不安全 trait */
    let my_struct = MyStruct::new(45, "light");
    my_struct.print(); // MyStruct: count = 45, name = light
    println!();

    /* 访问联合体中的字段 */
    let u = IntOrFloat { f: 1.0 };
    println!("now i is {}", unsafe { u.i }); // 1065353216
    println!("now f is {}", unsafe { u.f }); // 1

    /* 补充 */
    /* 内联汇编 */
    // asm!宏，可以在Rust代码中嵌入汇编代码
    // 支持以下架构:
    // - x86 和 x86-64
    // - ARM
    // - AArch64
    // - RISC-V

    // 基本用法
    unsafe {
        asm!("nop"); //插入一个NOP指令(空操作) 到编译器生成的汇编代码中
    }

    // 输入和输出
    let x: u64;
    unsafe {
        asm!("mov {}, 5", out(reg) x);
    }
    assert_eq!(x, 5);
    //
    // 首先，需要说明目标变量是作为内联汇编的输入还是输出，在本例中，是一个输出 out
    // 最后，要指定变量将要使用的寄存器，本例中使用通用寄存器 reg，编译器会自动选择合适的寄存器
    //
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!("mov {0}, {1}", "add {0}, 5", out(reg) o, in(reg) i);
    } //先把i的值复制到o，然后对o加5
    assert_eq!(o, 8);

    let mut x: u64 = 3;
    unsafe {
        asm!("add {0}, 5", inout(reg) x); // x既是输入也是输出
    }
    assert_eq!(x, 8);

    let x: u64 = 3;
    let y: u64;
    unsafe {
        asm!("add {0}, 5", inout(reg) x => y); // 指定不同的输入和输出
    }
    assert_eq!(y, 8);

    // 延迟输出操作数
    let mut a: u64 = 4;
    let b: u64 = 4;
    let c: u64 = 4;
    unsafe {
        asm!(
        "add {0}, {1}",
        "add {0}, {2}",
        inlateout(reg) a,
        in(reg) b,
        in(reg) c,
        );
    }
    assert_eq!(a, 12);

    // 显式指定寄存器
    /*let cmd = 0xd1;
    unsafe { //reg是通用寄存器，而eax是x86下的寄存器
        asm!("out 0x64, eax", in("reg") cmd);
    }*/
    println!("mul: 5 x 6 = {}", mul(5, 6));

    // Clobbered寄存器  无需作为输出的状态都会被内联汇编修改，这个状态被称之为 "clobbered"
    // three entries of four bytes each
    let mut name_buf = [0_u8; 12];
    // String is stored as ascii in ebx, edx, ecx in order
    // Because ebx is reserved, the asm needs to preserve the value of it.
    // So we push and pop it around the main asm.
    // (in 64 bit mode for 64 bit processors, 32 bit processors would use ebx)
    unsafe {
        asm!(
        "push rbx",
        "cpuid",
        "mov [rdi], ebx",
        "mov [rdi + 4], edx",
        "mov [rdi + 8], ecx",
        "pop rbx",
        // We use a pointer to an array for storing the values to simplify
        // the Rust code at the cost of a couple more asm instructions
        // This is more explicit with how the asm works however, as opposed
        // to explicit register outputs such as `out("ecx") val`
        // The *pointer itself* is only an input even though it's written behind
        in("rdi") name_buf.as_mut_ptr(),
        // select cpuid 0, also specify eax as clobbered
        inout("eax") 0 => _,
        // cpuid clobbers these registers too
        out("ecx") _,
        out("edx") _,
        );
    }
    let name = core::str::from_utf8(&name_buf).unwrap();
    println!("CPU Manufacturer ID: {}", name); // CPU Manufacturer ID: GenuineIntel

    // Multiply x by 6 using shifts and adds
    let mut x: u64 = 4;
    unsafe {
        asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);
}

unsafe fn dangerous() {
    println!("this is a unsafe function");
}

///
/// cannot borrow `*slice` as mutable more than once at a time
///
/*fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
}*/
/// 此时使用引用就无法走通，必须用指针
///
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
///
/// 从 C 代码中调用 Rust 接口
///
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

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;

    unsafe {
        asm!(
        // The x86 mul instruction takes rax as an implicit input and writes
        // the 128-bit result of the multiplication to rax:rdx.
        "mul {}",
        in(reg) a,
        inlateout("rax") b => lo,
        lateout("rdx") hi
        );
    }

    ((hi as u128) << 64) + lo as u128
}
