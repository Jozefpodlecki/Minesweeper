use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::{DomStringMap, HtmlElement};
use yew::MouseEvent;

pub trait DomStringMapExtensions {
    fn get_unchecked(&self, key: &str) -> String;
    fn parse_unchecked<T: FromStr>(&self, key: &str) -> T;
}

impl DomStringMapExtensions for DomStringMap {
    fn parse_unchecked<T: FromStr>(&self, key: &str) -> T {
        unsafe {
            self.get(key)
                .unwrap_unchecked()
                .parse::<T>()
                .ok()
                .unwrap_unchecked()
        }
    }
    
    fn get_unchecked(&self, key: &str) -> String {
        unsafe { self.get(key).unwrap_unchecked() }
    }
}

pub trait MouseEventExtensions {
    fn target_dataset_unchecked(&self) -> DomStringMap;
}

impl MouseEventExtensions for MouseEvent {
    fn target_dataset_unchecked(&self) -> DomStringMap {
        let current_target = unsafe { self.target().unwrap_unchecked() };
        let html_element = current_target.unchecked_into::<HtmlElement>();
        html_element.dataset()
    }
}