use web_sys::{window, Storage, Window};

use crate::models::Settings;

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsManager(Storage);

impl SettingsManager {
    pub fn new() -> Self {
        unsafe { 
            let window: Window = window().unwrap_unchecked();
            let storage: Storage = window.local_storage()
                .unwrap_unchecked()
                .unwrap_unchecked();
            Self(storage)
        }
    }

    pub fn get_or_create(&self) -> Settings {
        unsafe {
            let json = self.0.get_item("settings").unwrap_unchecked();
            let settings = json.and_then(|json| serde_json::from_str(&json).ok()).flatten().unwrap_or_default();
            settings   
        }
    }

    pub fn save(&self, value: Settings) {
        unsafe {
            let json = serde_json::to_string(&value).unwrap_unchecked();
            self.0.set_item("settings", &json).unwrap_unchecked();
        }
    }
}