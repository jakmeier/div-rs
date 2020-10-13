#![allow(unused_must_use)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    div::init_to("div-root");

    // Create a new pane at offset (100,100) from body
    // with size 500px/500px and then create a single
    // text node inside it with the text "Hello world"
    let x = 100;
    let y = 100;
    let w = 500;
    let h = 500;
    let html = "Hello world";
    div::new_pane(x, y, w, h, html);
}
