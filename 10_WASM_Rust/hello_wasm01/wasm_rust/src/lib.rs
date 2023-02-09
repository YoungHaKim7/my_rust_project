#[warn(stable_features)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_greeting() -> String {
    "Hello, world!".to_string()
}
#[wasm_bindgen]
pub fn generate_custom_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}
