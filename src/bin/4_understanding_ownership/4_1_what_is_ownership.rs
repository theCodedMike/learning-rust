/// 4.1 什么是所有权？
/// cargo r --bin 4_3
fn main() {
    /*
    #什么是所有权？
    ##内存管理方式
    - 方式一: GC  比如Java、Go
    - 方式二: 手动分配并释放内存   比如C++
    - 方式三: 通过所有权系统管理内存

    栈中的所有数据都必须占用已知且固定的大小   先入后出
    在堆上分配内存: 内存分配器（memory allocator）在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 指针（pointer）

    ##所有权规则
    - Rust中的每一个值都有一个被称为其 所有者（owner）的变量
    - 值在任一时刻有且只有一个所有者
    - 当所有者（变量）离开作用域，这个值将被丢弃

    ##变量作用域

    ##String类型

    ##内存与分配
    ###变量与数据交互的方式（一）：移动
    ###变量与数据交互的方式（二）：克隆
    ###只在栈上的数据：拷贝
    - 所有整数类型，比如 u32
    - 布尔类型，bool
    - 所有浮点数类型
    - 字符类型，char
    - 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32)
    - 不可变引用 &T，变引用 &mut T是不可以 Copy的

    ##所有权与函数

    ##返回值与作用域

     */
    {                      // s 在这里无效, 它尚未声明
        let s = "hello";   // 从此处起，s 开始有效

        // 使用 s
    }                      // 此作用域已结束，s 不再有效   Rust为我们调用一个特殊的函数drop
    let x = 5;  // 将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y  copy
    let y = x;
    println!("x = {}, y = {}", x, y);
    let s1 = String::from("hello");
    let s2 = s1; // String的所有权转移到s2，s1不再有效  该操作称为move
    //println!("{}", s1); // 编译报错，因为s1不再有效，否则会有二次释放(double free)问题
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 克隆，即 deep copy      shallow copy(浅拷贝)
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x

    let s1 = gives_ownership();         // gives_ownership 将返回值
    // 移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到
    // takes_and_gives_back 中,
    // 它也将返回值移给 s3

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
fn gives_ownership() -> String {           // gives_ownership 将返回值移动给
    // 调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string                              // 返回 some_string 并移出给调用的函数
}
// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}