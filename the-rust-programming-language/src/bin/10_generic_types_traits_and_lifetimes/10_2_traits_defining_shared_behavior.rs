#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

/// 10.2 trait: 定义共同的行为
///
/// cargo r --bin trait
///
/// ## 目录
/// ### 定义 trait
/// - 以一种抽象的方式来定义共享的行为，类似其他语言的接口概念，但还是有点不同
///
/// ### 为类型实现 trait
///
/// ### 默认实现
///
/// ### trait 作为参数
/// #### Trait Bound 语法
/// #### 通过 + 指定多个 trait bound
/// #### 通过 where 简化 trait bound
///
/// ### 返回实现了 trait 的类型
///
/// ### 使用 trait bounds 来修复 largest 函数
///
/// ### 使用 trait bound 有条件地实现方法
///
fn main() {
    /* 定义 trait */
    /* 为类型实现 trait */
    let article = NewsArticle {
        headline: "I have a dream".to_string(),
        location: "10086, 1st Ave, WDC, USA".to_string(),
        author: String::from("Mardin"),
        content: String::from("i have a day dream"),
    };
    println!("article is '{}'", article.summarize());
    println!();

    /* 默认实现 */
    let tweet = Tweet::build("json", "use as to simplify");
    println!("default impl: {}", tweet.default_method());
    println!();

    /* trait 作为参数 */
    notify(tweet);
    notify(article);
    println!();

    /* 特征约束(Trait Bound)语法 */
    let article = NewsArticle::new(
        "A Murder happened",
        "James",
        "a 68-year-old man was killed last night",
    );
    let tweet = Tweet::build("Marco", "a visit to Rome");
    notify_use_trait_bound(tweet.clone());
    notify_use_trait_bound(article.clone());
    //这里需要小心！！！
    notify_diff_struct(article.clone(), tweet.clone());
    notify_diff_struct(article.clone(), article.clone());
    //notify_same_struct(article.clone(), tweet.clone()); // 编译报错
    notify_same_struct(article.clone(), article.clone());

    /* 通过 + 指定多个 trait bound */
    let _ = some_function(5, "multi_trait");

    /* 通过 where 简化 trait bound */
    let _ = some_function_use_where(true, 6.4);
    println!();

    /* 返回实现了 trait 的类型 */
    let switch = rand::thread_rng().gen_range(1..=10) > 5;
    let summarize = returns_summarize(switch);
    println!("trait object: {}", summarize.summarize());
    println!();

    /* 使用 trait bound 有条件地实现方法 */
    let space = Space::new("CSS", "ISS");
    space.cmp_display(); // ISS
    println!();

    // 使用 trait bounds 来修复 largest 函数
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result); // 100
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result); // y
    println!();

    /* 补充 举例 */
    let p1 = Point {
        x: 1.1_f32,
        y: 1.1_f32,
    };
    let p2 = Point {
        x: 2.1_f32,
        y: 2.1_f32,
    };
    println!("new point: {:?}", add(p1, p2));
    let p3 = Point { x: 1, y: 1 };
    let p4 = Point { x: 2, y: 2 };
    println!("new point: {:?}", p3 + p4);
    let nf = File::new("nf.txt");
    println!("default style: {:?}", nf); // File { name: "nf.txt", data: [], state: Closed }
    println!("    new style: {}", nf); //   new style: <nf.txt (CLOSED)>

    // 通过derive派生特征  被derive标记的对象会自动实现对应的默认特征代码，实现相应的功能
    //
    // 默认格式的 Debug
    // 等值比较的 PartialEq 和 Eq
    // 次序比较的 PartialOrd 和 Ord
    // 复制值的 Clone 和 Copy
    // 固定大小的值映射的 Hash
    // 默认值的 Default
    //
}
/// 定义 trait
pub trait Summary {
    fn summarize(&self) -> String;
    // 默认方法，可以被重写
    fn default_method(&self) -> String {
        String::from("(Read more...)")
    }
}
#[derive(Clone)] // 自动实现clone方法
pub struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl NewsArticle {
    fn new(headline: &str, author: &str, content: &str) -> Self {
        NewsArticle {
            headline: String::from(headline),
            location: String::from("Sky Hotel"),
            author: author.to_string(),
            content: format!("{}", content),
        }
    }
}
/// 为类型实现 trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Clone)]
pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Tweet {
    fn build(username: &str, content: &str) -> Self {
        Tweet {
            username: String::from(username),
            content: content.to_string(),
            reply: false,
            retweet: true,
        }
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn default_method(&self) -> String {
        String::from("tweet news is fake news!!!")
    }
}

pub fn notify(item: impl Summary + Clone) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_use_trait_bound<T: Summary + Clone>(item: T) {
    println!("use trait bound, Breaking news! {}", item.summarize());
}
/// item1 和 item2 是不同的类型，也可以相同
///
/// 本质上如下，所以item1和item2可以是不同的类型
/// ```rust
/// pub fn notify_diff_struct<T1, T2>(item1: T1, item2: T2)
///     where T1: Summary,
///           T2: Summary
/// {
///
/// }
/// ```
pub fn notify_diff_struct(item1: impl Summary, item2: impl Summary) {
    println!("notify_diff_struct: {}", item1.default_method());
    println!("notify_diff_struct: {}", item2.default_method());
}
pub fn notify_diff_struct2<T1, T2>(item1: T1, item2: T2)
where
    T1: Summary,
    T2: Summary,
{
    println!("notify_diff_struct: {}", item1.default_method());
    println!("notify_diff_struct: {}", item2.default_method());
}

pub fn notify_same_struct<T: Summary>(item1: T, item2: T) {
    println!("notify_same_struct: {}", item1.default_method());
    println!("notify_same_struct: {}", item2.default_method());
}
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    5
}
fn some_function_use_where<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}
/* fn returns_summarize(switch: bool) -> impl Summary {  */
// 编译报错，`if` and `else` have incompatible types
// 如果只返回1种类型，则可以使用 impl Summary
fn returns_summarize(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best
            hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        })
    }
}
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
struct Space<T> {
    x: T,
    y: T,
}

impl<T> Space<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Space<T> {
    /// 只有实现了Display和PartialOrd特征的类型的实例，才能调用cmp_display方法
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    //限制类型T必须实现了Add特征，否则无法进行+操作
    x: T,
    y: T,
}
impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn add<T>(a: T, b: T) -> T
where
    T: Add<T, Output = T>,
{
    a + b
}

#[derive(Debug)]
pub enum FileState {
    Open,
    Closed,
}
impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
    pub state: FileState,
}
impl File {
    pub fn new(name: &str) -> File {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}
impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}
