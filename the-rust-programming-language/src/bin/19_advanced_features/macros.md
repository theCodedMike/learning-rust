## 标准库提供的声明宏(pub)
| \  | 声明宏                      | 含义                               | 备注               |
|----|--------------------------|----------------------------------|------------------|
| 1  | panic!                   | 用于使程序崩溃，比如判断索引超出下界后使程序直接崩溃，就可以使用 | 遇到不可恢复的错误时可以使用   |
| 2  | assert!                  | 判断表达式为true                       | dev、release环境下有用 |
| 3  | assert_eq!               | 判断2个表达式相等                        | dev、release环境下有用 |
| 4  | assert_ne!               | 判断2个表达式不相等                       | dev、release环境下有用 |
| 5  | debug_assert!            | 判断表达式为true                       | 仅dev环境下有用        |
| 6  | debug_assert_eq!         | 判断2个表达式相等                        | 仅dev环境下有用        |
| 7  | debug_assert_ne!         | 判断2个表达式不相等                       | 仅dev环境下有用        |
| 8  | matches!                 | 判断某个表达式是否匹配某个模式，返回值为true/false   |                  |
| 9  | try!                     | 传播错误                             | 已废弃,建议使用?来传播错误   |
| 10 | write!                   | 写入数据到buffer（末尾不换行）               |                  |
| 11 | writeln!                 | 写入数据到buffer（末尾换行）                |                  |
| 12 | unreachable!             | 表明无法执行到                          |                  |
| 13 | unimplemented!           | 表明未实现                            |                  |
| 14 | todo!                    | 表明将来会实现                          |                  |
| 15 | compile_error!           | 表明编译出错                           | 在自定义声明式宏时使用      |
| 16 | format_args!             | 格式化参数                            |                  |
| 17 | format!                  | 格式化参数并生成字符串                      |                  |
| 18 | env!                     | 获取环境变量(如果key不对直接panic)           |                  |
| 19 | option_env!              | 获取环境变量(返回值为Option)               |                  |
| 20 | concat_idents!           | 拼接标识符                            | unstable         |
| 21 | concat_bytes!            | 拼接字节                             | unstable         |
| 22 | concat!                  | 拼接多个字面量为一个字符串索引                  |                  |
| 23 | line!                    | 获取代码所在行数                         |                  |
| 24 | column!                  | 获取列数                             |                  |
| 25 | file!                    | 获取所在文件名                          |                  |
| 26 | stringify!               | 字符串化参数                           |                  |
| 27 | include_str!             | 导入文件(文件以字符串的形式存在)                |                  |
| 28 | include_bytes!           | 导入文件(文件以字节的形式存在)                 |                  |
| 29 | include!                 | 导入文件                             |                  |
| 30 | module_path!             | 模块名                              |                  |
| 31 | cfg!                     | 判断某个配置值，返回值为true/false           |                  |
| 32 | log_syntax!              |                                  | unstable         |
| 33 | trace_macros!            |                                  | unstable         |
| 34 | vec!                     | 创建动态数组                           |                  |
| 35 | print!                   | 打印到标准输出(末尾不换行)                   |                  |
| 36 | println!                 | 打印到标准输出(末尾会换行)                   |                  |
| 37 | eprint!                  | 打印到标准错误(末尾不换行)                   |                  |
| 38 | eprintln!                | 打印到标准错误(末尾会换行)                   |                  |
| 39 | dbg!                     | 打印值                              | dev、release环境下有用 |
| 40 | thread_local!            | 线程本地变量                           |                  |
| 41 | is_x86_feature_detected! | 判断x86/x86-64平台是否支持CPU的某个功能       |                  |