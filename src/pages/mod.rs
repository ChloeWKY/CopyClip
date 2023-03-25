use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod home;
pub mod index;
pub mod preferences;
pub mod search;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}