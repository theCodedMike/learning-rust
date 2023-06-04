use chrono::{Local, Utc};

///
/// cargo r --bin ch
///
fn main() {
    let utc_now = Utc::now();
    let local_now = Local::now();
    println!("  UTC: {}", utc_now);
    println!("LOCAL: {}", local_now);
}
