use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/package.js")]
extern "C" {
    pub fn html2canvas(element: JsValue) -> JsValue;
}
