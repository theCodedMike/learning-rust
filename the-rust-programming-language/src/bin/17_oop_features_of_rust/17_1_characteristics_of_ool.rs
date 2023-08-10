/// 17.1 面向对象语言的特点
///
/// cargo r --bin ool
///
/// ## 目录
/// ### 对象包含数据和行为
/// - 面向对象的程序是由对象组成的。一个对象包含数据和操作这些数据的过程。这些过程通常被称为方法或操作
/// - Rust中的结构体/枚举提供了与对象相同的功能
///
/// ### 封装(encapsulation)隐藏了实现细节
/// - 对象的实现细节不能被使用对象的代码获取到
///
/// ### 继承(Inheritance)，作为类型系统与代码共享
/// - 继承: 一个对象可以定义为继承另一个对象定义中的元素，这使其可以获得父对象的数据和行为，而无需重新定义
/// - 如果一个语言必须有继承才能被称为面向对象语言的话，那么 Rust 就不是面向对象的
/// - 多态(polymorphism):
///
fn main() {
    /* 对象包含数据和行为 */

    /* 封装隐藏了实现细节 */
    let mut col = AveragedCollection::new();
    println!("{:?}", col); // { list: [], average: 0.0 }

    col.add(5);
    col.add(10);
    col.add(-1);
    println!("{:?}", col); // { list: [5, 10, -1], average: 4.666666666666667 }

    col.remove();
    println!("{:?}", col); // { list: [5, 10], average: 7.5 }

    /* 继承，作为类型系统与代码共享 */
}

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn new() -> Self {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

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
