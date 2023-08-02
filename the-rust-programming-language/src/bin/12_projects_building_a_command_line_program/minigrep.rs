mod lib_crate;

use std::{env, process};

/// 12 一个I/O项目: 构建命令行程序
///
///   大小写敏感:  cargo r --bin minigrep  body  poem.txt
/// 大小写不敏感:  CASE_INSENSITIVE=1 cargo r --bin 12  body  poem.txt
///
/// 本来这是一个独立的项目，为了代码集中在一起，就不单独搞了
///
/// 如果想单独搞，使用 cargo new minigrep 命令创建一个新的package
///
/// crates.io里的ripgrep是一个更完善的版本，可以参考
/// # 目录:
/// ## 12.1 接受命令行参数
/// #### 读取参数值
/// #### 将参数值保存进变量
///
/// ## 12.2 读取文件
///
/// ## 12.3 重构改进模块性和错误处理
/// #### 二进制项目的关注分离
///
/// main 函数中的责任应该被限制为：
/// - 使用参数值调用命令行解析逻辑
/// - 设置任何其他的配置
/// - 调用 lib.rs 中的 run 函数
/// - 如果 run 返回错误，则处理这个错误
///
/// #### 提取参数解析器
/// #### 组合配置值
/// #### 创建一个 Config 的构造函数
/// #### 修复错误处理
/// #### 从 main 提取逻辑
/// #### 将代码拆分到库 crate
///
/// ## 12.4 采用测试驱动开发完善库的功能
/// - 测试驱动开发（Test Driven Development, TDD）
/// #### 编写失败测试
/// #### 编写使测试通过的代码
/// - 使用 lines 方法遍历每一行
/// - 用查询字符串搜索每一行
/// - 存储匹配的行
/// - 在 run 函数中使用 search 函数
///
/// ## 12.5 处理环境变量
/// #### 编写一个大小写不敏感 search 函数的失败测试
///
/// ## 12.6 将错误信息输出到标准错误而不是标准输出
/// #### 检查错误应该写入何处
/// ```
/// cargo run > output.txt
/// > 语法告诉 shell 将标准输出的内容写入到 output.txt 文件中而不是屏幕上
/// ```
/// #### 将错误打印到标准错误
/// ```
/// cargo r --bin 12  body  poem.txt > output.txt
/// 可以将println!打印的内容输出到output.txt文件中
/// ```
fn main() {
    // 这里用到了一个闭包（closure）
    let config = lib_crate::Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // 非零的退出状态是一个惯例，用来告诉调用程序的进程：该程序以错误状态退出
        process::exit(1)
    });
    if let Err(e) = lib_crate::run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}
