use sha3::{Digest, Sha3_256};

const DELIMITER: &str = "     ";
const SALT: &str = "I_STILL_LOVE_YOU";

///
/// cargo r --bin sha
///
fn main() {
    let hello = format!("{}{}{}", "123456", DELIMITER, SALT);
    let result = Sha3_256::digest(<String as AsRef<[u8]>>::as_ref(&hello));
    println!("{:X}", result);
}
