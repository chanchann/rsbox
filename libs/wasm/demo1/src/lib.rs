extern crate wasm_bindgen;
mod models;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn echo() -> String {
    format!("{}", "chanchan")
}

// 我们想直接使用js里面的console.log
#[wasm_bindgen]
extern {
    // console.log(xxx)
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn log_hello() {
    log("hello");
}

#[wasm_bindgen]
pub fn log_param(s: &str) {
    log(s);
}

macro_rules! echo_macro {
    ($expr:expr) => (
        log(format!("{}", $expr).as_str());
    )
}

#[wasm_bindgen]
pub fn log_macro(s: &str) {
    echo_macro!(s);
}

