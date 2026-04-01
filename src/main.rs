// #![allow(unused_imports)]
// #![allow(unused_variables)]

mod app;
mod api;
mod models;
mod pages;
mod game;
mod components;
mod route;
mod extensions;
mod services;
mod utils;
mod ffi;
mod log_level;

// #[cfg(test)]
mod testing;

use app::App;
use log::Level;
use wasm_bindgen::JsCast;
use wasm_logger::{init, Config};
use web_sys::window;
use yew::Renderer;

use crate::{app::AppProps, log_level::LocalStorageLogLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = window().expect("Windows object not found");
    let local_storage = window.local_storage().ok().flatten().expect("Local storage not found");
    let document = window.document().expect("Document object not found");
    let body = document.body().expect("Document body not found");
    let root = document.get_element_by_id("root").expect("Root element not found");
    let default_background = document.get_element_by_id("default-background").expect("Default background not found");
    let navigator = window.navigator();
    let log_level = LocalStorageLogLevel::new(local_storage.clone());

    log_level.set_default_if_missing();
    init(Config::new(log_level.get_default()));
    console_error_panic_hook::set_once();

    let app_name = env!("CARGO_PKG_NAME").into();
    let version = env!("CARGO_PKG_VERSION").into();

    let props = AppProps {
        app_name,
        version,
        window,
        document,
        body: body.clone(),
        default_background: default_background.unchecked_into(),
        local_storage,
        navigator
    };

    let renderer = Renderer::<App>::with_root_and_props(root, props);
    renderer.render();

    Ok(())
}
