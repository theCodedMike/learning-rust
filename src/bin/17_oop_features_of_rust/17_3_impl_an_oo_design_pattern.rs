use std::task::ready;
use ansi_term::{Colour};
use serde::de::Unexpected::Str;

/// 17.3 实现面向对象设计模式
///
/// cargo r --bin 17_3
///
/// ## 目录:
/// ### 定义 Post 并新建一个草案状态的实例
/// ### 存放博文内容的文本
/// ### 确保博文草案的内容是空的
/// ### 请求审核博文来改变其状态
/// ### 增加改变 content 行为的 approve 方法
/// ### 状态模式的权衡取舍
/// ### 将状态和行为编码为类型
/// ### 实现状态转移为不同类型的转换
/// - 在 Rust 中面向对象模式并不总是最好的解决方案
///
fn main() {
    let mut post = Post::new();
    post.request_review();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.approve();
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // 定义 Post 并新建一个草案状态的实例
    // 存放博文内容的文本
    // 确保博文草案的内容是空的
    // 请求审核博文来改变其状态
    // 增加改变 content 行为的 approve 方法
    let mut post = PostUseTrait::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // 状态模式的权衡取舍
    // 将状态和行为编码为类型
    // 实现状态转移为不同类型的转换
    let mut post = PostUseStruct::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

}
/******************************普通模式****************************/
pub struct Post {
    text: String,
    state: State
}
#[derive(Debug, PartialEq, PartialOrd)]
pub enum State {
    Empty,
    Processing,
    Finish,
    Publish
}
impl Post {
    pub fn new() -> Self {
        Post {
            text: String::new(),
            state: State::Empty
        }
    }
    /// 添加草案，如果是第1次添加，则状态将转换为 处理中
    pub fn add_text(&mut self, text: &str) {
        match self.state {
            State::Empty => {
                self.text.push_str(text);
                self.state = State::Processing;
            },
            State::Processing => {
                self.text.push_str(text);
            },
            State::Finish => {
                println!("{}", Colour::Red.bold().paint("Post had been finished, can't add text"));
            },
            State::Publish => {
                println!("{}", Colour::Red.bold().paint("Post had been published, can't add text"));
            }
        }
    }
    /// 请求审核博文，意味着状态将转变为 已完成
    pub fn request_review(&mut self) {
        match self.state {
            State::Empty => {
                println!("{}", Colour::Red.bold().paint("Post is empty, can't request review"));
            },
            State::Processing => {
                self.state = State::Finish;
            },
            State::Finish => {
                println!("{}", Colour::Green.bold().paint("Post is reviewing, don't request again"));
            },
            State::Publish => {
                println!("{}", Colour::Red.bold().paint("Post had been published, can't request review"));
            }
        }
    }
    /// 批准
    pub fn approve(&mut self) {
        match self.state {
            State::Empty => {
                println!("{}", Colour::Red.bold().paint("Post is empty, can't approve"));
            },
            State::Processing => {
                println!("{}", Colour::Red.bold().paint("Post is processing, can't approve"));
            },
            State::Finish => {
               self.state = State::Publish;
            },
            State::Publish => {
                println!("{}", Colour::Red.bold().paint("Post had been published, can't approve"));
            }
        }
    }
    pub fn content(&self) -> &str {
        match self.state {
            State::Publish => {
                &self.text
            },
            _ => {
                ""
            }
        }
    }
}


/******************************状态模式****************************/
pub struct PostUseTrait {
    state: Option<Box<dyn StateTrait>>,
    content: String,
}
trait StateTrait {
    fn request_review(self: Box<Self>) -> Box<dyn StateTrait>;
    fn approve(self: Box<Self>) -> Box<dyn StateTrait>;
    fn content<'a>(&self, post: &'a PostUseTrait) -> &'a str {
        ""
    }
}
/// 草稿
struct Draft {}
impl StateTrait for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn StateTrait> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn StateTrait> {
        self
    }
}
/// 等待审核
struct PendingReview {}
impl StateTrait for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn StateTrait> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn StateTrait> {
        Box::new(Published {})
    }
}
/// 审核通过/可以发表
struct Published {}
impl StateTrait for Published {
    fn request_review(self: Box<Self>) -> Box<dyn StateTrait> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn StateTrait> {
        self
    }

    fn content<'a>(&self, post: &'a PostUseTrait) -> &'a str {
        &post.content
    }
}
impl PostUseTrait {
    /// 定义 Post 并新建一个草案状态的实例
    pub fn new() -> Self {
        PostUseTrait {
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }
    /// 存放博文内容的文本
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    /// 确保博文草案的内容是空的
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    /// 请求审核博文来改变其状态
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    /// 增加改变 content 行为的 approve 方法
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}


/******************************循环模式****************************/
pub struct PostUseStruct {
    content: String,
}
pub struct DraftPost {
    content: String,
}
pub struct PendingReviewPost {
    content: String,
}
impl PostUseStruct {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}
impl PendingReviewPost {
    pub fn approve(self) -> PostUseStruct {
        PostUseStruct {
            content: self.content,
        }
    }
}