use web_sys::{window, Storage, Window};

use crate::models::Settings;

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsManager(Storage);

impl SettingsManager {
    pub fn new(storage: Storage) -> Self {
        Self(storage)
    }

    pub fn init(&self) {
        unsafe {
            let json = self.0.get_item("settings").unwrap_unchecked();
            let settings: Option<Settings> = json.and_then(|json| serde_json::from_str(&json).ok()).flatten();

            if settings.is_none() {
                let settings = settings.unwrap_or_default();
                let json = serde_json::to_string(&settings).unwrap_unchecked();
                self.0.set_item("settings", &json).unwrap_unchecked();
            }
            
        }
    }

    pub fn get(&self) -> Settings {
        unsafe {
            let json = self.0.get_item("settings").unwrap_unchecked();
            let settings = json.and_then(|json| serde_json::from_str(&json).ok()).flatten().unwrap_unchecked();
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