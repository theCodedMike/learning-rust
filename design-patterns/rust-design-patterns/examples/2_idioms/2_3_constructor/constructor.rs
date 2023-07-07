use getset::{Getters, Setters};

#[derive(Debug)]
pub struct Second {
    value: u64,
}
impl Second {
    pub fn new(value: u64) -> Self {
        Second { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}
/// 实现Default特征
impl Default for Second {
    fn default() -> Self {
        Second { value: 100 }
    }
}
/// 派生宏Default
#[derive(Debug, Default, Getters, Setters)]
#[getset(get = "pub with_prefix", set = "pub")]
pub struct Country {
    name: String,
    code: u16,
}
