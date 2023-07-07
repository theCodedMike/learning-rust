//! ## 把入参包在返回值Err中
//!
//! ### 描述
//! 在一个容易出错的函数中，返回值Err中要携带上入参信息
//!
//! ### 优点
//!
//! ### 缺点
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 2_15
//!

fn main() {
    let mut value = "imagine this is very long string".to_string();

    let success = 's: {
        // Try to send value two times.
        for _ in 0..2 {
            value = match send(value) {
                Ok(()) => break 's true,
                Err(SendError(value)) => value,
            }
        }
        false
    };

    println!("success: {}", success);
}
/// send函数容易发生错误
///
/// 所以返回值Err中携带上入参value
pub fn send(value: String) -> Result<(), SendError> {
    println!("using {value} in a meaningful way");

    // Simulate non-deterministic fallible action.
    use std::time::SystemTime;
    let period = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    if period.subsec_nanos() % 2 == 1 {
        Ok(())
    } else {
        Err(SendError(value))
    }
}

pub struct SendError(String);
