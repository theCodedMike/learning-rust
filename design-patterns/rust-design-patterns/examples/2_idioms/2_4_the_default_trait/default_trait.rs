use getset::Getters;
pub use std::path::PathBuf;
use std::time::Duration;

#[derive(Default, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct MyConfiguration {
    // Option defaults to None
    output: Option<PathBuf>,
    // Vec default to empty vector
    search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
    // i/u_8/16/32/64 defaults to 0
    age: usize,
    // f_32/64 defaults to 0.0
    height: f64,
    // char defaults to '\0'
    ch: char,
    // String defaults to ""
    name: String,
}
