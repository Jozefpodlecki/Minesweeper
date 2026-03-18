#![allow(unused_imports)]
#![allow(unused_variables)]

mod app;
mod app_state;
mod api;
mod models;
mod pages;
mod components;
mod route;

use app::App;
use log::Level;
use wasm_logger::{init, Config};
use yew::Renderer;

fn main() {
    let log_level = if cfg!(debug_assertions) {
        Level::Debug
    } else {
        Level::Info
    };

    init(Config::new(log_level));
    console_error_panic_hook::set_once();
    Renderer::<App>::new().render();
}
