## 11.1 如何编写测试
单元测试示例代码见: src/lib.rs

集成测试示例代码见: tests/integration_test.rs

### 测试函数剖析
- 测试就是一个带有 test 属性标注的函数
- test是属性，之前还见过derive
- 使用 #[test] 属性标明哪些函数是测试函数
- 单元测试、集成测试、基准测试
- cargo test 命令即可执行单元测试、基准测试代码

### 使用 assert! 宏来检查结果

### 使用 assert_eq! 和 assert_ne! 宏来测试相等

### 自定义失败信息

### 使用 should_panic 检查 panic

### 将 Result<T, E> 用于测试

## 11.2 控制测试如何运行
- 运行 cargo test --help 会提示 cargo test 的有关参数
```
cargo test [OPTIONS] [TESTNAME] [-- [args]...]

Arguments:
[TESTNAME]  If specified, only run tests containing this string in their names
[args]...   Arguments for the test binary

Options:
-q, --quiet                   Display one character per test instead of one line
    --lib                     Test only this package's library unit tests
    --bin [<NAME>]            Test only the specified binary
    --bins                    Test all binaries
-v, --verbose...              Use verbose output (-vv very verbose/build.rs output)
    --example [<NAME>]        Test only the specified example
    --color <WHEN>            Coloring: auto, always, never
    --examples                Test all examples
    --frozen                  Require Cargo.lock and cache are up to date
    --test [<NAME>]           Test only the specified test target
    --locked                  Require Cargo.lock is up to date
    --tests                   Test all tests
    --bench [<NAME>]          Test only the specified bench target
    --offline                 Run without accessing the network
    --benches                 Test all benches
    --config <KEY=VALUE>      Override a configuration value
    --all-targets             Test all targets
-Z <FLAG>                     Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details       
    --doc                     Test only this library's documentation
    --no-run                  Compile, but don't run tests
    --no-fail-fast            Run all tests regardless of failure
-p, --package [<SPEC>]        Package to run tests for
    --workspace               Test all packages in the workspace
    --exclude <SPEC>          Exclude packages from the test
    --all                     Alias for --workspace (deprecated)
-j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs
    --keep-going              Do not abort the build as soon as there is an error (unstable)
-r, --release                 Build artifacts in release mode, with optimizations
    --profile <PROFILE-NAME>  Build artifacts with the specified profile
-F, --features <FEATURES>     Space or comma separated list of features to activate
    --all-features            Activate all available features
    --no-default-features     Do not activate the `default` feature
    --target <TRIPLE>         Build for the target triple
    --target-dir <DIRECTORY>  Directory for all generated artifacts
    --manifest-path <PATH>    Path to Cargo.toml
    --ignore-rust-version     Ignore `rust-version` specification in packages
    --message-format <FMT>    Error format
    --unit-graph              Output build graph in JSON (unstable)
    --future-incompat-report  Outputs a future incompatibility report at the end of the build
    --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json
-h, --help                    Print help

```
- 运行 cargo test -- --help 可以提示在分隔符 -- 之后使用的有关参数
```
Options:
    --include-ignored         Run ignored and not ignored tests
    --ignored                 Run only ignored tests
    --force-run-in-process    Forces tests to run in-process when panic=abort
    --exclude-should-panic    Excludes tests marked as should_panic
    --test                    Run tests and not benchmarks
    --bench                   Run benchmarks instead of tests
    --list                    List all tests and benchmarks
-h, --help                    Display this message
    --logfile PATH            Write logs to the specified file
    --nocapture               don't capture stdout/stderr of each task, allow printing directly
    --test-threads n_threads  Number of threads used for running tests in parallel
    --skip FILTER             Skip tests whose names contain FILTER (this flag can be used multiple times)
-q, --quiet                   Display one character per test instead of one line. Alias to --format=terse
    --exact                   Exactly match filters rather than by substring
    --color auto|always|never           Configure coloring of output
    --format pretty|terse|json|junit    Configure formatting of output
    --show-output             Show captured stdout of successful tests
-Z unstable-options           Enable nightly-only flags:
    --report-time             Show execution time of each test.
    --ensure-time             Treat excess of the test execution time limit as error.
    --shuffle                 Run tests in random order
    --shuffle-seed SEED       Run tests in random order; seed the random number generator with SEED

```

### 并行或连续的运行测试
```markdown
将测试线程数量设置为1。默认是给每个测试函数分配1个线程
cargo test -- --test-threads=1
```

### 显示函数输出
```markdown
显示所有标准输出，不管该测试函数是否成功:
cargo test -- --show-output
```

### 通过指定名字来运行部分测试
#### 运行单个测试
```markdown
cargo test 测试函数名

eg: cargo test exploration
```

#### 过滤运行多个测试
```markdown
cargo test 部分测试函数名

eg: cargo test gr

可以匹配到2个测试函数：greeting_contains_name、greater_than_100
```

### 忽略某些测试
```markdown
忽略某个测试函数: #[ignore]

执行所有被忽略的测试函数: cargo test --ignored
```

## 11.3 测试的组织结构
- 单元测试（unit tests）: 倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口
- 集成测试（integration tests）: 测试公有接口且每个测试都有可能会测试多个模块

### 单元测试
- 单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中
- 规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块

#### 测试模块和 #[cfg(test)]
- 测试模块的 #[cfg(test)] 标注告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做
- cfg 属性代表 configuration

#### 测试私有函数
```markdown
cargo test internal
```
### 集成测试
- 只能调用一部分库中的公有 API
- 目的是测试库的多个部分能否一起正常工作

#### tests 目录
- tests 目录与 src 同级
```markdown
仅执行集成测试中的integration_test:

cargo test --test integration_test
```
#### 集成测试中的子模块
- tests 目录中的子目录不会被作为单独的 crate 编译或作为一个测试结果部分出现在测试输出中

#### 二进制 crate 的集成测试
- 只有库 crate 才会向其他 crate 暴露可供调用和使用的函数；二进制 crate 只能单独运行