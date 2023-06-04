use derive_builder::Builder;

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct Glass {
    name: Option<String>,
    length: Option<f64>,
    radius: Option<u32>,
    color: Option<Color>,
}
#[derive(Debug, Clone)]
pub enum Color {
    Green,
    Blue,
    Red,
}
