use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

const PASSWORD: &str = "123456";

///
/// cargo r --bin ar
///
fn main() {
    let hash = generate_password_hash();

    validate_password_hash(&hash);
}

fn generate_password_hash() -> String {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let password_hash = Argon2::default()
        .hash_password(PASSWORD.as_ref(), &salt)
        .expect("Failed to hash password")
        .to_string();

    println!("\n\ngenerate_password_hash:\n {}", password_hash);

    password_hash
}

fn validate_password_hash(password_hash: &String) {
    let hash = PasswordHash::new(password_hash).expect("Failed to new PasswordHash");

    let result = Argon2::default().verify_password(PASSWORD.as_bytes(), &hash);

    println!("\n\nvalidate_password_hash: {}", result.is_ok()); // true
}
