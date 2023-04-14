## 使用包、Crate和模块管理不断增长的项目
- 包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
- Crates ：一个模块的树形结构，它形成了库或二进制项目。
- 模块（Modules）和 use：允许你控制作用域和路径的私有性。
- 路径（path）：一个命名例如结构体、函数或模块等项的方式

### 7.1 包和Crate
1、包（package）是提供一系列功能的一个或者多个 crate。

2、一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate

3、一个包中至多只能包含一个库 crate（library crate）,可以包含任意多个二进制 crate（binary crate）

4、src/main.rs：与包同名的二进制crate

5、src/lib.rs：与包同名的库crate

6、通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate


### 7.2 定义模块来控制作用域与私有性
1、模块 让我们可以将一个 crate 中的代码进行分组，以提高可读性与重用性

2、模块还可以控制项的 私有性，即项是可以被外部代码使用的（public），还是作为一个内部实现的内容，不能被外部代码使用（private）

示例代码如下: src/lib.rs:
```rust
mod front_of_house {
    mod hosting {
        fn add_to_wait_list() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

3、模块树:
```
crate(lib.rs)
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### 7.3 路径用于引用模块树中的项
1、路径有两种形式：
- 绝对路径（absolute path）从 crate 根部开始，以 crate 名或者字面量 crate 开头
- 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头

2、绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符

3、Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的

4、父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项

5、模块上的 pub 关键字只允许其父模块引用它

示例代码如下: src/lib.rs:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_wait_list();

    // 相对路径
    front_of_house::hosting::add_to_wait_list();
}
```
6、使用 super 开头来构建从父模块开始的相对路径，这么做类似于文件系统中以 .. 开头的语法

示例代码如下: src/lib.rs:
```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```
7、在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的

示例代码如下: src/lib.rs
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天点一份黑麦面包作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释，将会导致编译失败；我们不被允许
    // 看到或更改随餐搭配的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}
```
8、将枚举设为公有，则它的所有成员都将变为公有

示例代码如下: src/lib.rs
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```


### 7.4 使用 use 关键字将名称引入作用域
1、使用 use 简化调用路径，类似于在文件系统中创建软连接（符号连接，symbolic link）

示例代码如下: src/lib.rs
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}
```
2、还可以使用 use 和相对路径来将一个项引入作用域。

示例代码如下: src/lib.rs
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}
```
3、使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径

示例代码如下: src/lib.rs
```rust
use std::collections::HashMap;
use std::fmt;
use std::io;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}
fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
```
4、使用 use 将两个同名类型引入同一作用域时可以用 as 指定一个新的本地名称或者别名

示例代码如下: src/lib.rs
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```
5、使用 pub use 重导出名称

示例代码如下: src/lib.rs
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}
// 使用后lib.rs以外的文件就可以调用add_to_wait_list()函数了
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}
```

6、使用外部包

示例代码如下: Cargo.toml
```
[dependencies]
rand = "0.8.3"
```
示例代码如下: src/main.rs
```rust
use std::collections::*; // 可以使用*导入全部
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let map = HashMap::new();
}
```
7、使用嵌套路径来消除大量的 use 行

示例代码如下: src/main.rs
```
use std::cmp::Ordering;
use std::io;
// ---snip---

等价于

use std::{cmp::Ordering, io};
// ---snip---
```


### 7.5 将模块分割进不同文件
1、各个模块可以移到单独的文件中

示例代码如下: src/lib.rs
```
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}
```

方式一：src/front_of_house.rs
```rust
pub mod hosting {
    pub fn add_to_wait_list() {
        
    }
}
```
方式二：src/front_of_house/hosting.rs
```rust
pub fn add_to_wait_list() {
    
}
```