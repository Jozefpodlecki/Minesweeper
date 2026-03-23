use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/ffi/ffi.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub fn html2canvas(element: JsValue) -> Result<Promise, JsValue>;
}
