/// 17.2 为使用不同类型地值而设计的特征对象
///
/// cargo r --bin 17_2
///
/// ## 目录:
/// ### 定义通用行为的 trait
/// ### 实现 trait
/// ### trait 对象执行动态分发
/// - 静态分发（static dispatch）: 指编译器在编译时就知道该调用什么方法，一般发生在单态化时
/// - 动态分发（dynamic dispatch）: 指编译器在编译时不知道调用什么方法，在运行时才确定调用什么方法
/// - 当使用 trait object时，Rust 必须使用动态分发
/// ### Trait 对象要求对象安全
/// 一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的：
/// - 返回值类型不为 Self
/// - 方法没有任何泛型类型参数
///
fn main() {
    // 定义通用行为的 trait
    // 实现 trait
    let mut screen = Screen::new();

    let select_box = SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No")
        ],
    };
    screen.push(select_box);
    let button = Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    };
    screen.push(button);

    screen.run();

    // trait 对象执行动态分发
    // Trait 对象要求对象安全


}
pub trait Draw {
    fn draw(&self);
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
        println!("this is button");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("this is select_box");
    }
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn new() -> Self {
        Screen {
            components: vec![]
        }
    }
    pub fn push(&mut self, component: impl Draw + 'static) {
        self.components.push(Box::new(component));
    }
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}