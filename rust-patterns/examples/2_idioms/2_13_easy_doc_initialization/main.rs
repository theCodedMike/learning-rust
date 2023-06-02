//! ## 文档初始化
//!
//! ### 描述
//! 如果一个结构体初始化操作很复杂，当写文档的时候，可以在文档中写一个使用样例的函数
//!
//! 有时候结构体有多个或者很复杂的参数和一堆方法。每个方法都应该有相应的例子说明
//!
//! ### 优点
//!
//! ### 缺点
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_13
//!

/// ```rust
/// struct Connection {
///     name: String,
///     stream: TcpStream,
/// }
///
/// impl Connection {
///     /// Sends a request over the connection.
///     ///
///     /// # Example
///     /// ```no_run
///     /// # // Boilerplate are required to get an example working.
///     /// # let stream = TcpStream::connect("127.0.0.1:34254");
///     /// # let connection = Connection { name: "foo".to_owned(), stream };
///     /// # let request = Request::new("RequestId", RequestType::Get, "payload");
///     /// let response = connection.send_request(request);
///     /// assert!(response.is_ok());
///     /// ```
///     fn send_request(&self, request: Request) -> Result<Status, SendErr> {
///         // ...
///     }
///
///     /// Oh no, all that boilerplate needs to be repeated here!
///     fn check_status(&self) -> Status {
///         // ...
///     }
/// }
/// ```
fn main() {
    println!("cargo r --example 2_13");
}
