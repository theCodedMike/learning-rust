use getset::{Getters, Setters};
/// Getters     -> get
/// MutGetters  -> get_mut
/// CopyGetters -> get_copy
/// Setters     -> set
#[derive(Debug, Default, Getters, Setters)]
#[getset(get = "pub with_prefix", set = "pub")]
pub struct Person {
    name: String,
    age: u16,
    school: String,
    token: f64,
    play_game: bool,
}
