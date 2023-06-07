use getset::{Getters, Setters};
/// Getters     -> get
/// MutGetters  -> get_mut
/// CopyGetters -> get_copy
/// Setters     -> set
///
/// pub 表示生成的getset方法是公开的
/// with_prefix 表示getset方法是否加前缀
#[derive(Debug, Default, Getters, Setters)]
#[getset(get = "pub with_prefix", set = "pub")]
pub struct Person {
    name: String,
    age: u16,
    school: String,
    token: f64,
    play_game: bool,
}
