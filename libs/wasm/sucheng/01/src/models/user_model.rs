extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct UserModel {
    user_id: i32,
}
#[wasm_bindgen]
impl UserModel {
    #[wasm_bindgen(getter)]
    pub fn uid(&self) -> i32 {
        self.user_id
    }
    #[wasm_bindgen(setter)]
    pub fn set_uid(&mut self, id: i32) {
        self.user_id = id;
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> UserModel {
        UserModel { user_id: -1 }
    }
}
#[wasm_bindgen]
pub fn new_user(id: i32) -> UserModel {
    UserModel { user_id: id }
}
