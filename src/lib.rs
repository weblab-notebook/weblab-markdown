use wasm_bindgen::prelude::*;

use comrak::{markdown_to_html, ComrakOptions};

#[wasm_bindgen]
pub fn markdown(input: &str) -> String {
    markdown_to_html(input, &ComrakOptions::default())
}
