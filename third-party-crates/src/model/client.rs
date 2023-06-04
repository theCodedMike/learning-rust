use getset::{Getters, Setters};

#[derive(Debug, Default, Getters, Setters)]
#[getset(get = "pub with_prefix", set = "pub")]
pub struct Client {
    host: String,
    port: u16,
}
impl Client {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }
}
