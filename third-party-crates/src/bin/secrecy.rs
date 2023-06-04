use secrecy::Secret;
use third_party_crates::model::SchoolBoy;

///
/// cargo r --bin se
///
fn main() {
    let boy = SchoolBoy {
        name: "mario".to_string(),
        age: 19,
        school: "Harvard".to_string(),
        token: Secret::new("token19230".to_string()),
    };

    println!("{:?}", boy);
    // SchoolBoy { name: "mario", age: 19, school: "Harvard", token: Secret([REDACTED alloc::string::String]) }
}
