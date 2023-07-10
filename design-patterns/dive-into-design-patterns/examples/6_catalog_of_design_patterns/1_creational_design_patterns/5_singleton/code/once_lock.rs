#![allow(dead_code)]
// use std::cell::OnceCell;
use std::sync::OnceLock; // This type is a thread-safe OnceCell

static mut USER_CONTROLLER: OnceLock<UserController> = OnceLock::new();
static CELL: OnceLock<OrderService> = OnceLock::new();

struct OrderService;

impl OrderService {
    fn get_singleton() -> &'static Self {
        CELL.get_or_init(|| OrderService)
    }

    fn name(&self) {
        println!("This is OrderService");
    }
}

#[derive(Debug)]
struct UserController {
    count: usize,
}

impl UserController {
    fn get_singleton() -> &'static mut Self {
        unsafe {
            // 这里无法锁住，失败
            if USER_CONTROLLER.get_mut().is_none() {
                USER_CONTROLLER
                    .set(UserController { count: 0 })
                    .expect("Failed to set");
            }
            USER_CONTROLLER.get_mut().unwrap()
        }
    }

    fn print(&self, thread_id: usize) {
        println!("ThreadId: {:4}, now count is {}", thread_id, self.count);
    }

    fn add_one(&mut self) {
        self.count += 1;
    }
}

/// cargo r --example singleton-once-lock
fn main() {
    // 无法锁住，失败
    /*let mut handlers = vec![];

    (1..=1000).for_each(|i| {
        let handle = std::thread::spawn(move || {
            let user = UserController::get_singleton();
            user.add_one();
            user.print(i);
        });
        handlers.push(handle);
    });

    handlers.into_iter().for_each(|h| h.join().unwrap());*/

    let order = OrderService::get_singleton();
    order.name();
}
