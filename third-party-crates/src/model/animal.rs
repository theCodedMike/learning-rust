use derive_builder::Builder;

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct Animal {
    name: String,
    weight: f32,
    length: f32,
    width: f32,
    height: f32,
    location: String,
    eat_meat: bool,
}
