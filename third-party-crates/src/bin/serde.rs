use serde::Deserialize;
use serde::Serialize;

///
/// cargo r --bin sd
///
fn main() {
    println!("hello serde");

    let person = Person {
        name: "xiaoming".to_string(),
        age: 19,
        school: "A University".to_string(),
        sex: Gender::Female,
        play_game: true,
    };

    println!("{}", serde_json::to_string_pretty(&person).unwrap());

    let string = serde_json::to_string_pretty(&person).unwrap();

    let de_person = serde_json::from_str::<Person>(&string).unwrap();

    println!("{:?}", de_person);
}
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: usize,
    school: String,
    sex: Gender,
    play_game: bool,
}
#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}
