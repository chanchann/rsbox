extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod components;
use components::App;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    yew::start_app::<App>();
    Ok(())
}