use std::{error::Error, fmt::{self, Debug, Display, Formatter}};

use serde::Deserialize;
use wasm_bindgen::JsValue;

use crate::models::FetchError;

#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    Loading,
    Error(FetchError),
    Loaded(Social)
}

impl Default for AppState {
    fn default() -> Self {
        Self::Loading
    }
}
    

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Social {
    pub linkedin: String,
    pub github: String,
    pub portfolio: String
}