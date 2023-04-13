/// 2.0 猜数字游戏
/// cargo r --bin 2
use std::cmp::Ordering;
use std::io;
use ansi_term::Colour;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    'outer: loop {
        let random = rand::thread_rng().gen_range(1..101);

        // 猜数字
        loop {
            println!("Please input your guess, and input 'exit' or 'quit' to quit");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read guess");
            if guess.contains("exit") || guess.contains("quit") {
                break 'outer;
            }
            let guess_num = match guess.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}{}{}",
                             Colour::Red.paint("********"),
                             Colour::Green.bold().paint("WARN: ONLY INPUT NUMBER"),
                             Colour::Red.paint("********")
                    );
                    continue
                }
            };
            println!("Your guess num is {}, and random is {}", guess_num, random);
            match guess_num.cmp(&random) {
                Ordering::Less => {
                    println!("{}", Colour::Purple.paint("Tip: Too small!\n"));
                },
                Ordering::Equal => {
                    println!("{}", Colour::Red.paint("You win!"));
                    break
                },
                Ordering::Greater => {
                    println!("{}", Colour::Yellow.paint("Tip: Too big!\n"));
                }
            }
        }
        println!("再来一局...");
    }
}
