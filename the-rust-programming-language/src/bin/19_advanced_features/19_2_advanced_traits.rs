use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref};

/// 19.2 高级特征
///
/// cargo r --bin advanced-traits
///
/// ## 目录
/// ### 关联类型在 trait 定义中指定占位符类型
///   - 关联类型（associated types）： 是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型
///
/// ### 默认泛型类型参数(default type parameters)和运算符重载(Operator overloading)
///   - 当使用泛型类型参数时，可以为泛型指定一个默认的具体类型
///   - 为泛型类型指定默认类型的语法是在声明泛型类型时使用 <PlaceholderType=ConcreteType>
///   - Rust 并不允许创建自定义运算符或重载任意运算符
///
/// ### 完全限定语法(fully qualified syntax)与消除歧义: 调用相同名称的方法
/// > <Type as Trait>::function(receiver_if_method, next_arg, ...);
///
/// ### supertrait 用于在另一个 trait 中使用某 trait 的功能
///   - supertrait: 表示在给struct实现某个trait之前，必须先实现的trait，即为supertrait，
///   - subtrait:
///
/// ### newtype 模式(newtype pattern)用于在外部类型上实现外部 trait
///   - 使用一个元组结构体来绕开孤儿规则
///
fn main() {
    /* 关联类型在 trait 定义中指定占位符类型 */
    pub trait Iterator {
        type Item; // 这里就是关联类型, 关联类型可以替代泛型, 如果使用泛型, 实现起来会非常麻烦
        fn next(&mut self) -> Option<Self::Item>;
    }

    /* 默认泛型类型参数 和 运算符重载 */
    // // RHS: right hand side
    // pub trait Add<Rhs = Self> {
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    let millimeters = Millimeters(5) + Meters(23);
    println!("{:?}", millimeters); // Millimeters(23005)
    println!();

    /* 完全限定语法与消除歧义: 调用相同名称的方法 */
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    println!("A baby dog is called a {}", Dog::baby_name()); // Spot
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // puppy
    println!();

    /* 父trait(supertrait) 用于在另一个 trait 中使用某 trait 的功能 */
    // pub trait FnOnce<Args: Tuple> {}
    // pub trait FnMut<Args: Tuple>: FnOnce<Args> {}
    // pub trait Fn<Args: Tuple>: FnMut<Args> {}
    let point = Point { x: 10, y: 20 };
    point.outline_print();
    println!();

    /* newtype 模式用于在外部类型上实现外部 trait */
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w); // w = [hello, world]
}
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot: This is your pilot speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard: This is your wizard speaking.");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
// 这里用到了supertrait，即实现OutlinePrint特征之前必须先实现了Display特征
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);
impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// 实现了Deref特征，这样就能直接使用Vec的方法了
impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
