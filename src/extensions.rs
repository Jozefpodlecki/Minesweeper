use std::str::FromStr;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{DomStringMap, HtmlElement, Node};
use yew::{MouseEvent, NodeRef};

pub trait DomStringMapExtensions {
    fn get_unchecked(&self, key: &str) -> String;
    fn parse_unchecked<T: FromStr>(&self, key: &str) -> T;
    fn set_unchecked(&self, name: &str, value: &str);
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
    
    fn set_unchecked(&self, name: &str, value: &str) {
        unsafe { self.set(name, value).unwrap_unchecked() }
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

pub trait ExtensionsNodeRef {
    fn unchecked_cast<T: AsRef<Node> + From<JsValue>>(&self) -> T;
}

impl ExtensionsNodeRef for NodeRef {
    fn unchecked_cast<T: AsRef<Node> + From<JsValue>>(&self) -> T {
        unsafe { self.cast::<T>().unwrap_unchecked() }
    }
}