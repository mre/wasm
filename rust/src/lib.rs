use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Hello, World!".into());
    Ok(())
}