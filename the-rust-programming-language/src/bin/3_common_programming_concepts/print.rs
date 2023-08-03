/// 以下内容皆为补充
///
/// cargo r --bin print
///
/// ## 目录
/// ### print! println! eprint! eprintln!
///
/// ### {} 与 {:?} 占位符
///
/// ### 位置参数
///
/// ### 具名参数
///
/// ### 格式化参数
/// #### 宽度
/// #### 对齐
/// #### 精度
/// #### 进制
/// #### 指数
/// #### 指针地址
/// #### 转义
///
fn main() {
    /* 补充 */
    /* print!，println!，format! */
    //
    //   print! 将格式化文本输出到标准输出，不带换行符
    // println! 同上，但是在行的末尾添加换行符
    //  format! 将格式化文本输出到 String 字符串
    //
    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");

    // eprint!，eprintln! 输出到标准错误
    eprintln!("Error: Could not complete task");
    println!();

    /* {} 与 {:?} 占位符 */
    //
    //   {} 适用于实现了std::fmt::Display特征的类型
    // {:?} 适用于实现了std::fmt::Debug特征的类型，用于调试场景
    // {:#?} 同2, 但可以美化输出
    //
    let i = std::f64::consts::PI;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}, {}, {:?}, {:?}", i, s, v, p);
    //3.141592653589793, "hello", [1, 2, 3], Person { name: "sunface", age: 18 }
    println!();

    /* 位置参数 */
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    println!("{1}{}{0}{}", 1, 2); // => 2112
    println!();

    /* 具名参数  带名称的参数必须放在不带名称参数的后面 */
    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
                                                      //println!("{abc} {1}", abc = "def", 2); //编译无法通过
    println!();

    /* 格式化参数 */
    let v = std::f64::consts::PI; //显示到小数点后2位
    println!("{:.2}", v); // Display => 3.14
    println!("{:.2?}", v); // Debug => 3.14
    println!("{:.2}", 5); // 5
    println!("{:.2}", 0.00055); // 0.00
    println!();

    /* 宽度 用来指示输出目标的长度，如果长度不够，则进行填充和对齐  {:x} */
    /* 字符串填充: 字符串格式化默认使用空格进行填充，并且进行左对齐 */
    println!("Hello {:5}!", "x"); // 为"x"后面填充空格，补齐宽度5                     => Hello x    !
    println!("Hello {:1$}!", "x", 5); // 使用参数5来指定宽度                         => Hello x    !
    println!("Hello {1:0$}!", 5, "x"); // 使用x作为占位符输出内容，同时使用5作为宽度     => Hello x    !
    println!("Hello {:width$}!", "x", width = 5); // 使用有名称的参数作为宽度          => Hello x    !
    println!("Hello {:1$}!{}", "x", 5); // 使用参数5为参数x指定宽度，同时在结尾输出参数5   => Hello x    !5
    println!("Hello {:5}!", "xxxxxxxxxx"); // => Hello xxxxxxxxxx!
    println!();

    /* 数字填充:符号和0  数字格式化默认也是使用空格进行填充，但数字是右对齐 */
    println!("Hello 0{:5}!", 5); // 宽度是5              => Hello 0    5!
    println!("Hello 0{:+}!", 5); // 显式的输出正号         => Hello 0+5!
    println!("Hello 0{:05}!", 5); // 宽度5，使用0进行填充   => Hello 00005!
    println!("Hello 0{:05}!", -5); // 负号也要占用一位宽度   => Hello 0-0005!
    println!("Hello 0{:5}!", 5555555); // 宽度是5 =>Hello 05555555!
    println!();

    /* 对齐 */
    println!("Hello {:<5}!", "x"); // 左对齐   => Hello x    !
    println!("Hello {:>5}!", "x"); // 右对齐   => Hello     x!
    println!("Hello {:^5}!", "x"); // 居中对齐  => Hello   x  !
    println!("Hello {:&<5}!", "x"); // 对齐并使用指定符号填充 => Hello x&&&&! 指定符号填充的前提条件是必须有对齐字符
    println!();

    /* 精度 用于控制浮点数的精度或者字符串的长度 */
    let v = std::f64::consts::PI;
    println!("{:.2}", v); // 保留小数点后两位       => 3.14
    println!("{:+.2}", v); // 带符号保留小数点后两位  => +3.14  减号(-)默认肯定会带，加号(+)如需显示则需要加+
    println!("{:.0}", v); // 不带小数             => 3
    println!("{:.1$}", v, 4); // 通过参数来设定精度  => 3.1416，相当于{:.4}
    let s = "hi我在自学Rust";
    println!("{:.3}", s); // 保留字符串前三个字符     => hi我
    println!("Hello {:.*}!", 3, "abcdefg"); // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!();

    /* 进制 可以使用 # 号来控制数字的进制输出 */
    println!("{:#b}!", 27); // 二进制 => 0b11011!
    println!("{:b}!", 27); // 二进制 =>   11011!
    println!("{:#o}!", 27); // 八进制 => 0o33!
    println!("{}!", 27); // 十进制 => 27!
    println!("{:#x}!", 27); // 小写十六进制 => 0x1b!
    println!("{:#X}!", 27); // 大写十六进制 => 0x1B!
    println!("{:x}!", 27); // 不带前缀的十六进制 => 1b!
    println!("{:#010b}", 27); // 使用0填充二进制，宽度为10 => 0b00011011
    println!("{:010b}", 27); // 使用0填充二进制，宽度为10 => 0000011011
    println!();

    /* 指数 */
    println!("{:e}", 100); // => 1e2
    println!("{:E}", 1000000000); // => 1E9
    println!("{:e}", 100.294723); // => 1.00294723e2
    println!();

    /* 指针地址 */
    let v = vec![1, 2, 3];
    let v_point = v.as_ptr();
    println!("pointer: {:p}", v_point); // => 0x24e911b0680
    println!();

    /* 转义 */
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    println!("Hello \"{{World}}\" "); // => Hello "{World}"
    println!();

    /* 在格式化字符串时捕获环境中的值(Rust 1.58 新增) */
    let person = get_person();
    println!("Hello, {person}!"); // => Hello, world!
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//为自定义类型实现 Display 特征
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Custom Print, Person.name = {}, Person.age = {}",
            self.name, self.age
        )
    }
}

fn get_person() -> String {
    String::from("world")
}
