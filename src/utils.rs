use std::rc::Rc;

use chrono::Utc;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::extensions::DomStringMapExtensions;

pub fn set_document_version(version: &str, document: &Document) {
    unsafe {
        let document_element = document.document_element().unwrap_unchecked();
        let document_element = document_element.unchecked_into::<HtmlElement>();
        document_element.dataset().set_unchecked("version", &version);
    }
}

pub fn timestamped_filename() -> String {
    let timestamp = Utc::now().format("%y%m%d%H%M%S").to_string();
    format!("{}.jpg", timestamp)
}