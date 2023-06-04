use third_party_crates::model::Person;

///
/// cargo r --bin gs
///
fn main() {
    let mut person = Person::default();
    println!("{:?}\n\n", person);
    // Person { name: "", age: 0, school: "", token: 0.0, play_game: false }

    person.set_name("flying bird".into());
    println!("{:?}\n\n", person);
    // Person { name: "flying bird", age: 0, school: "", token: 0.0, play_game: false }

    let mut wang_ming = Person::default();
    wang_ming
        .set_name("Wang ming".into())
        .set_token(3.4)
        .set_age(33);
    println!("{:?}", wang_ming.get_play_game());
}
