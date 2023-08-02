/// 17.1 面向对象语言的特点
///
/// cargo r --bin ool
///
/// ## 目录:
/// ### 对象包含数据和行为
/// - 结构体/枚举提供了与对象相同的功能
///
/// ### 封装隐藏了实现细节
///
/// ### 继承，作为类型系统与代码共享
/// - 如果一个语言必须有继承才能被称为面向对象语言的话，那么 Rust 就不是面向对象的
///
fn main() {
    // 对象包含数据和行为
    // 封装隐藏了实现细节
    // 继承，作为类型系统与代码共享
}
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
