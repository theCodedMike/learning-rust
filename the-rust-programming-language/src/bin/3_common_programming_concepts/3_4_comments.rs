#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::time::Duration;

/// 3.4 注释
///
/// cargo r --bin comments
///
/// ## 补充
/// ### 注释的种类
///   1. 代码注释: 用于说明某一块代码的功能，读者往往是同一个项目的协作开发者
///   2. 文档注释: 对项目描述、公共API等用户关心的功能进行介绍，同时还能提供示例代码，目标读者往往是想要了解你项目的人
///   3. 包和模块注释: 主要用于说明当前包和模块的功能
///
/// ### 代码注释
///   - 行注释: //
///   - 块注释: /* ..... */
///
/// ### 文档注释(struct、enum、method、function等)
///   - 行注释: ///
///   - 块注释: /** ... */
///
/// ### 包和模块注释(要添加到包、模块的最上方)
///   - 行注释: //!
///   - 块注释: /*! ... */
///
fn main() {
    // 我们在这里处理一些复杂事情，需要足够长的解释，使用
    // 多行注释可做到这点。哇！我们希望这个注释将解释接下
    // 来要实现的内容。
    /*
    1 line
    2 line
    3 line
     */
    let lucky_number = 7; // I’m feeling lucky today

    let mut map = HashMap::new();
    map.insert("Li Si", Student::new("Li Si", 18, Gender::Female));
    map.insert("Zhang San", Student::new("Zhang San", 20, Gender::Male));
}

///
/// 性别枚举
///
enum Gender {
    Male,   // 男性
    Female, // 女性
}

///
/// 学生
///
struct Student {
    name: String, // 姓名
    age: u16,     // 年龄
    sex: Gender,  // 性别
}

impl Student {
    /// 创建学生实例
    fn new(name: &str, age: u16, sex: Gender) -> Self {
        Student {
            name: name.into(),
            age,
            sex,
        }
    }

    /// 是否成年
    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    /// 做作业
    fn do_homework(&self) {
        // do something
        // again
        // ...
        // after 1 hour
        // finish
        std::thread::sleep(Duration::from_secs(3));
    }
}
