use validator::validate_email;

///
/// cargo r --bin va
///
fn main() {
    println!("{}", validate_email("1243802@qq.com"));

    let email = "di9kdfo@";
    let parts = email.rsplitn(2, '@').collect::<Vec<&str>>();
    println!("len is {}", parts.len()); // 2

    for (idx, item) in parts.iter().enumerate() {
        println!("{}: {}", idx, item);
    }
}
