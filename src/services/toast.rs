use web_sys::{window, Storage, Window};

use crate::models::{AppError, Settings};

#[derive(Debug, Clone, PartialEq)]
pub struct ToastManager(Window);

impl ToastManager {
    pub fn new(window: Window) -> Self {
        Self(window)
    }

    pub fn send(&self, error: AppError) {
        
    }
}