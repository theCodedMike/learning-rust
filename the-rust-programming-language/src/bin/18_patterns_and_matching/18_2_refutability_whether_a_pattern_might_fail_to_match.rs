/// 18.2 模式是否会匹配失败
///
/// cargo r --bin fail-to-match
///
/// ## 目录
/// - 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）
/// - 不可反驳的（irrefutable）: 能匹配任何传递的可能值的模式
/// - 可反驳的（refutable）: 对某些可能的值进行匹配会失败的模式
/// - 函数参数、 let 语句和 for 循环只能接受不可反驳的模式
/// - if let 和 while let 表达式被限制为只能接受可反驳的模式
fn main() {}
