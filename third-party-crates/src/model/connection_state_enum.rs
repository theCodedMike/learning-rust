#[derive(Debug)]
pub enum ConnectionStateEnum<'a> {
    None,
    Connecting(i32),
    Connected(&'a str),
    Disconnecting(bool),
    Disconnected(String),
}
