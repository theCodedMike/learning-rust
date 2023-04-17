/// 13.2 使用迭代器处理元素系列
///
/// cargo r --bin 13_2
///
/// ## 目录:
/// - 迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束
/// - 在 Rust 中，迭代器是 惰性的（lazy），也就是说在调用 消费适配器 前不会有效果
/// ### Iterator trait 和 next 方法
/// ```
/// pub trait Iterator {
///     type Item; // 关联类型（associated type）
///
///     fn next(&mut self) -> Option<Self::Item>;
///
///     // 此处省略了方法的默认实现
/// }
/// ```
/// - 消费适配器（consuming adaptors）：调用 next 方法的方法，他们会消耗迭代器。如sum()
/// - 迭代器适配器（iterator adaptors）：可以将当前迭代器变为不同类型的迭代器。如map()
/// ### 使用闭包获取环境
/// ### 实现 Iterator trait 来创建自定义迭代器
/// #### 使用 Counter 迭代器的 next 方法
/// #### 使用自定义迭代器中其他 Iterator trait 方法
///
fn main() {
    // Iterator trait 和 next 方法
    let v1 = vec![1, 2, 3];
    // let mut v1_iter = v1.into_iter();  // 这里会转移v1的所有权
    let mut v1_iter = v1.iter(); // 这里获取到v1的不可变引用
    // let mut v1_iter = v1.iter_mut();   // 这里获取到v1的可变引用
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // 消费迭代器的方法: 消费型适配器
    let v1 = vec![1, 2, 3];
    let sum = v1.iter().sum::<i32>();
    assert_eq!(sum, 6);

    // 产生其他迭代器的方法: 迭代型适配器，不会触发next操作
    let v1 = vec![1, 2, 3];
    let _ = v1.iter().map(|x| x + 1);

    // 使用闭包获取环境
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );

    // 实现 Iterator trait 来创建自定义迭代器
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    let sum: u32 = Counter::new()                                  // 1、2、3、4、5
        .zip(Counter::new().skip(1))  // (1, 2)、(2, 3)、(3, 4)、(4, 5)
        .map(|(a, b)| a * b)       // 2、6、12、20
        .filter(|x| x % 3 == 0)         // 6、12
        .sum();                                                    // 18
    assert_eq!(18, sum);

}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32; //必须指明 关联类型 的类型

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}