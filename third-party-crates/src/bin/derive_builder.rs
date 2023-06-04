use third_party_crates::model::{Animal, AnimalBuilder, GlassBuilder};

///
/// cargo r --bin db
///
fn main() {
    let panda = AnimalBuilder::default()
        .name("Panda".to_string())
        .location("Sichuan province".to_string())
        .length(1.2)
        .width(3.4)
        .height(1.5)
        .build()
        .unwrap();
    println!("{:?}\n\n", panda);

    let bird = Animal::default();
    println!("{:?}\n\n", bird);

    let glass = GlassBuilder::default()
        .name(Some("LargeGlass".into()))
        .radius(Some(10))
        .build()
        .unwrap();
    println!("{:?}", glass);
}
