use rand::Rng;
use std::cmp::Ordering;

/// 9.3 使用 panic! 还是不用 panic!
///
/// cargo r --bin error
///
/// ## 目录
/// ### 示例、代码原型和测试都非常适合 panic
///
/// ### 当我们比编译器知道更多的情况
///   使用 unwrap 或 expect 是恰当的
///
/// ### 错误处理指导原则
///   - 当有可能会导致有害状态的情况下建议使用 panic!, 例如无效的值、自相矛盾的值或者被传递了不存在的值
///   - 如果别人调用你的代码并传递了一个没有意义的值，尽最大可能返回一个错误
///   - 当错误预期会出现时，返回 Result 仍要比调用 panic! 更为合适
///
/// ### 创建自定义类型进行有效性验证
///
/// ### 总结
///   - panic! 宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理
///   - Result 枚举代表操作可能会在一种可以恢复的情况下失败
///
fn main() {
    /* 创建自定义类型进行有效性验证 */
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        // --snip--

        println!("Please input your guess.");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
