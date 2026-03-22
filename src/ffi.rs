use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/package.js")]
extern "C" {
    fn html2canvas() -> String;
}
