use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

pub fn set_document_version(value: &()) {
    unsafe {
        let document = window().unwrap_unchecked().document().unwrap_unchecked();
        let document_element = document.document_element().unwrap_unchecked();
        let document_element = document_element.unchecked_into::<HtmlElement>();
        let version = env!("CARGO_PKG_VERSION");
        document_element.dataset().set("version", version).unwrap_unchecked();
    }
}