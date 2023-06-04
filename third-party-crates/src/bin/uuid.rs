use uuid::Uuid;

///
/// cargo r --bin uu
///
fn main() {
    let string = Uuid::new_v4().to_string();
    println!("{}", string);

    let mut query = String::new();
    let a = [3, 8, 5, 1, 8, 5, 3, 2, 7];
    let mut idx = 0;

    while idx < a.len() {
        if a[idx] % 2 != 0 {
            query += (a[idx] + a[a[idx]]).to_string().as_str();
            idx += 2;
        } else {
            idx -= 1;
        }
    }
    println!("url: {}", query);
}
