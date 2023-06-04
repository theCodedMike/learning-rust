use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::{Fake, Faker};

///
/// cargo r --bin fk
///
fn main() {
    let faker: String = Faker.fake();
    let subject: String = Sentence(1..5).fake();
    let content: String = Paragraph(1..10).fake();
    let fake_email: String = SafeEmail().fake();
    println!("{}\n\n", faker);
    println!("{}\n\n", subject);
    println!("{}\n\n", content);
    println!("{}\n\n", fake_email);
}
