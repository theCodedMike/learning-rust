// use std::cell::OnceCell;
use std::sync::OnceLock; // This type is a thread-safe OnceCell

static USER_CONTROLLER: OnceLock<UserController> = OnceLock::new();

struct UserController;

impl UserController {
    fn get_singleton() -> &'static Self {
        USER_CONTROLLER.get_or_init(|| UserController)
    }

    fn name(&self) {
        println!("this is UserController");
    }
}

/// cargo r --example singleton-once-lock
fn main() {
    let singleton = UserController::get_singleton();

    singleton.name();
}
