## 进一步认识 Cargo 和 Crates.io
- 使用发布配置来自定义构建
- 将库发布到 crates.io
- 使用工作空间来组织更大的项目
- 从 crates.io 安装二进制文件
- 使用自定义的命令来扩展 Cargo

Cargo的全部功能请查看 [Cargo手册](https://rustwiki.org/zh-CN/cargo/)

### 14.1 采用发布配置自定义构建
- Cargo 有两个主要的配置：运行 cargo build 时采用的 dev 配置和运行 cargo build --release 的 release 配置 
> cargo build   
> cargo build --release

- 当项目的 Cargo.toml 文件中没有任何 [profile.*] 部分的时候，Cargo 会对每一个配置都采用默认设置
> [profile.dev]   
> opt-level = 1  // 可以控制Rust对代码进行何种程度的优化
> 
> [profile.release]  
> opt-level = 3  // 范围是0-3，值越大，需要更多的时间编译

- 每个配置的设置和其默认值的完整列表，请查看
[Cargo之profile](https://rustwiki.org/zh-CN/cargo/reference/manifest.html#the-profile-sections)

### 14.2 将 crate 发布到 Crates.io
- 编写有用的文档注释( 2种: /// 、 /** ... */ )
```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
最终效果如下:   
![文档注释](../../../assets/doc_comments.png)

- 常用（文档注释）部分
> Panics: 这个函数可能会 panic! 的场景   
> Errors: 如果这个函数返回 Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误   
> Safety: 如果这个函数使用 unsafe 代码，这一部分应该会涉及到期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件（invariants）

- 文档注释作为测试
> cargo test 也会执行文档注释中的示例代码
> cargo doc --open 可以查看文档

- 注释包含项的结构( 2种: //! 、 /*! ... */ )
> 通常用于 crate 根文件（通常是 src/lib.rs）或模块的根文件，为 crate 或模块整体提供文档
```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
最终效果如下:   
![包含项的文档注释](../../../assets/commenting_contained_items.png)

- 使用 pub use 导出合适的公有 API
```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Purple
    }
}
```
最终效果如下:   
![re-export](../../../assets/pub_use.png)

- 创建 Crates.io 账号
> 1、使用GitHub账号注册/登录crates.io网站   
> 2、查看位于 https://crates.io/me/ 的账户设置页面并获取 API token   
> 3、cargo login token  该命令会通知 Cargo 你的 API token，并将其储存在本地的 ~/.cargo/credentials 文件中

- 发布新 crate 之前
> 1、设置 package.name、description、license等元信息   
> 2、cargo publish

- 发布到 Crates.io
> cargo publish

- 发布现存 crate 的新版本
> 1、使用 语义化版本规则 来根据修改的类型决定下一个版本号   
> 2、cargo publish

- 使用 cargo yank 从 Crates.io 撤回版本
> cargo yank --vers 1.0.1   


### 14.3 Cargo 工作空间
- 创建工作空间
> 工作空间 是一系列共享同样的 Cargo.lock 和输出目录的包   

```markdown
示例代码如下：

├── Cargo.lock
├── Cargo.toml  (*)
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs  (bin)
└── target

[workspace] (*)
members = ["adder"]
```
- 在工作空间中创建第二个 crate
```markdown
示例代码如下:

├── Cargo.lock
├── Cargo.toml  (*)
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs   (lib)
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs  (bin)
└── target

[workspace] (*)
members = ["adder", "add-one"]
[dependencies] (adder)
add-one = { path = "../add-one" }
```
- 在工作空间中依赖外部 crate
```markdown
[dependencies] (add-one)
rand = "0.5.5"
```

- 为工作空间增加测试

> 1、发布工作空间中的 crate 时，工作空间中的每一个 crate 需要单独发布   
> 2、cargo publish 命令并没有 --all 或者 -p 参数，所以必须进入每一个 crate 的目录并运行 cargo publish 来发布工作空间中的每一个 crate


### 14.4 使用 cargo install 从 Crates.io 安装二进制文件
- cargo install 命令用于在本地安装和使用二进制 crate   
- 只有拥有二进制目标文件的包能够被安装   
- 二进制目标 文件是在 crate 有 src/main.rs 或者其他指定为二进制文件时所创建的可执行程序   
- cargo install安装的二进制文件安装在 $HOME/.cargo/bin 这一路径

### 14.5 Cargo 自定义扩展命令
Cargo已有命令如下:
```markdown
    add                  Add dependencies to a Cargo.toml manifest file
    b                    alias: build
    bench                Execute all benchmarks of a local package
    build                Compile a local package and all of its dependencies
    c                    alias: check
    check                Check a local package and all of its dependencies for errors
    clean                Remove artifacts that cargo has generated in the past
    clippy               Checks a package to catch common mistakes and improve your Rust code.
    config               Inspect configuration values
    d                    alias: doc
    doc                  Build a package's documentation
    fetch                Fetch dependencies of a package from the network
    fix                  Automatically fix lint warnings reported by rustc
    fmt                  Formats all bin and lib files of the current crate using rustfmt.
    generate-lockfile    Generate the lockfile for a package
    git-checkout         This command has been removed
    help                 Displays help for a cargo subcommand
    init                 Create a new cargo package in an existing directory
    install              Install a Rust binary. Default location is $HOME/.cargo/bin
    locate-project       Print a JSON representation of a Cargo.toml file's location
    login                Save an api token from the registry locally. If token is not specified, it will be read from stdin.
    logout               Remove an API token from the registry locally
    metadata             Output the resolved dependencies of a package, the concrete used versions including overrides, in machine-readable format
    miri
    new                  Create a new cargo package at <path>
    owner                Manage the owners of a crate on the registry
    package              Assemble the local package into a distributable tarball
    pkgid                Print a fully qualified package specification
    publish              Upload a package to the registry
    r                    alias: run
    read-manifest        Print a JSON representation of a Cargo.toml manifest.
    remove               Remove dependencies from a Cargo.toml manifest file
    report               Generate and display various kinds of reports
    rm                   alias: remove
    run                  Run a binary or example of the local package
    rustc                Compile a package, and pass extra options to the compiler
    rustdoc              Build a package's documentation, using specified custom flags.
    search               Search packages in crates.io
    t                    alias: test
    test                 Execute all unit and integration tests and build examples of a local package
    tree                 Display a tree visualization of a dependency graph
    uninstall            Remove a Rust binary
    update               Update dependencies as recorded in the local lock file
    vendor               Vendor all dependencies for a project locally
    verify-project       Check correctness of crate manifest
    version              Show version information
    yank                 Remove a pushed crate from the index

```