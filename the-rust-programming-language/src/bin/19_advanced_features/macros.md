## 标准库中的声明式宏
| \  | macro             | 备注               |
|----|-------------------|------------------|
| 1  | panic             | 遇到不可恢复的错误时可以使用   |
| 2  | assert            | dev、release环境下有用 |
| 3  | assert_eq         | dev、release环境下有用 |
| 4  | assert_ne         | dev、release环境下有用 |
| 5  | debug_assert      | 仅dev环境下有用        |
| 6  | debug_assert_eq   | 仅dev环境下有用        |
| 7  | debug_assert_ne   | 仅dev环境下有用        |
| 8  | matches           |                  |
| 9  | try               | 已废弃,建议使用?来传播错误   |
| 10 | write             |                  |
| 11 | writeln           |                  |
| 12 | unreachable       |                  |
| 13 | unimplemented     |                  |
| 14 | todo              |                  |
| 15 | compile_error     | 在自定义声明式宏时使用      |
| 16 | format_args       |                  |
| 17 | format            |                  |
| 18 | const_format_args | 无法在代码中使用         |
| 19 | format_args_nl    | 无法在代码中使用         |
| 20 | env               |                  |
| 21 | option_env        |                  |
| 22 | concat_idents     | unstable         |
| 23 | concat_bytes      | unstable         |
| 24 | concat            |                  |
| 25 | line              |                  |
| 26 | column            |                  |
| 27 | file              |                  |
| 28 | stringify         |                  |
| 29 | include_str       |                  |
| 30 | include_bytes     |                  |
| 31 | include           |                  |
| 32 | module_path       |                  |
| 33 | cfg               |                  |
| 34 | log_syntax        | unstable         |
| 35 | trace_macros      | unstable         |
| 36 | vec               |                  |
| 37 | print             |                  |
| 38 | println           |                  |
| 39 | eprint            |                  |
| 40 | eprintln          |                  |
| 41 | dbg               | dev、release环境下有用 |