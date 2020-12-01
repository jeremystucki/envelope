extern crate wasm_bindgen;

use envelope_core::create_test_file;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test() -> Vec<u8> {
    create_test_file()
}
