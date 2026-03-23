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

use app::App;
use log::Level;
use wasm_logger::{init, Config};
use web_sys::window;
use yew::Renderer;

use crate::app::AppProps;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = if cfg!(debug_assertions) {
        Level::Debug
    } else {
        Level::Info
    };

    let window = window().expect("Windows object not found");
    let local_storage = window.local_storage().ok().flatten().expect("Local storage not found");
    let document = window.document().expect("Document object not found");
    let body = document.body().expect("Document body not found");
    let navigator = window.navigator();
    // let clipboard = navigator.clipboard();

    init(Config::new(log_level));
    console_error_panic_hook::set_once();

    let props = AppProps {
        window,
        document,
        body: body.clone(),
        local_storage,
        navigator
    };

    let renderer = Renderer::<App>::with_root_and_props(body.into(), props);
    renderer.render();

    Ok(())
}
