extern crate wasm_bindgen;

use envelope_core::{Envelope, PaperSize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_envelope(sender: String, recipient: String) -> Vec<u8> {
    let mut output_buffer = Vec::new();

    Envelope::new_with_size(PaperSize::C5)
        .sender(sender)
        .recipient(recipient)
        .write(&mut output_buffer);

    output_buffer
}
