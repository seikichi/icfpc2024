use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u64 {
    base::add(u64::from(a), u64::from(b))
}
