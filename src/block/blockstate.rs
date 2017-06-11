use std::collections::HashMap;

#[derive(Deserialize)]
pub struct BlockState {
    variants: HashMap<String, Variant>,
}

#[derive(Deserialize)]
pub struct Variant {
    model: String,
    x: i32,
    y: i32,
    uvlock: bool,
    weight: i32,
}
