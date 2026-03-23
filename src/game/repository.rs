use wasm_bindgen::JsError;
use web_sys::{window, Storage};

use crate::{game::SavedGameState, models::{GameResult}};


#[derive(Debug, Clone, PartialEq)]
pub struct Repository {
    items: Vec<GameResult>,
    local_storage: Storage
}

impl Repository {
    pub fn new(local_storage: Storage) -> Self {

        let items = unsafe {
            let json = local_storage.get_item("records").unwrap_unchecked();
            let records = json.and_then(|json| serde_json::from_str(&json).unwrap_unchecked());
            records.unwrap_or_default()
        };

        Self {
            items,
            local_storage
        }
    }

    pub fn clear_records(&mut self) {
        unsafe { self.local_storage.remove_item("records").unwrap_unchecked(); }
    }

    pub fn get_last_records<'a>(&'a self) -> &'a [GameResult] {
        // self.items.sort_by_key(|r| r.created_on);
        let len = self.items.len();
        let start = len.saturating_sub(5);

        &self.items[start..]
    }

    pub fn set_last_record(&mut self, value: GameResult) {
        unsafe {
            self.items.push(value);

            if self.items.len() > 5 {
                self.items.remove(0);
            }

            let json = serde_json::to_string(&self.items).unwrap_unchecked();
            self.local_storage.set_item("records", &json).unwrap_unchecked();
        }
    }

    pub fn save_game_session(&self, value: SavedGameState) {
        unsafe {
            let json = serde_json::to_string(&value).unwrap_unchecked();
            self.local_storage.set_item("state", &json).unwrap_unchecked();
        }
    }

    pub fn get_last_game_session(&self) -> Option<SavedGameState> {
        unsafe {
            let json = self.local_storage.get_item("state").unwrap_unchecked();
            let state = json.map(|json| serde_json::from_str(&json).unwrap_unchecked());
            state
        }
    }
}