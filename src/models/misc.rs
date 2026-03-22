use std::{error::Error, fmt::{self, Debug, Display, Formatter}};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::models::AppError;

#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    Loading,
    Error(AppError),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub created_on: DateTime<Utc>,
    pub duration: u32,
    pub has_won: bool,
    pub revealed_mines: u32,
    pub total_mines: u32
}