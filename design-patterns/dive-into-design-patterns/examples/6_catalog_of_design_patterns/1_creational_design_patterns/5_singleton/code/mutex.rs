//! ructc 1.63
//! https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
//!
//! Starting with Rust 1.63, it can be easier to work with global mutable
//! singletons, although it's still preferable to avoid global variables in most
//! cases.
//!
//! Now that `Mutex::new` is `const`, you can use global static `Mutex` locks
//! without needing lazy initialization.

use std::sync::{Mutex, MutexGuard, PoisonError};

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

static USER_CONTROLLER: Mutex<UserController> = Mutex::new(UserController { count: 0 });

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

struct UserController {
    count: usize,
}
impl UserController {
    fn get_singleton() -> Result<MutexGuard<'static, Self>, PoisonError<MutexGuard<'static, Self>>>
    {
        USER_CONTROLLER.lock()
    }

    fn add_one(&mut self) {
        self.count += 1;
    }

    fn print(&self, thread_id: usize) {
        println!("ThreadId: {:4}, now count is {}", thread_id, self.count);
    }
}

/// cargo r --example singleton-mutex
fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("Called {} times", ARRAY.lock().unwrap().len());

    let mut handlers = vec![];

    for i in 1..=1000 {
        let handle = std::thread::spawn(move || match UserController::get_singleton() {
            Ok(mut lock) => {
                lock.add_one();
                lock.print(i);
            }
            Err(_) => {
                println!("ThreadId: {:4}, failed to get lock", i);
            }
        });
        handlers.push(handle);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());

    UserController::get_singleton().unwrap().add_one();
    UserController::get_singleton().unwrap().print(0);
}
