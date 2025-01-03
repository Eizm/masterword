#![recursion_limit = "10000"]
mod home;

use wasm_bindgen::prelude::*;
use wasm_logger;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<home::Home>::new().render();
}
