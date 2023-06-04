use secrecy::Secret;

#[derive(Debug)]
pub struct SchoolBoy {
    pub name: String,
    pub age: u16,
    pub school: String,
    pub token: Secret<String>,
}
