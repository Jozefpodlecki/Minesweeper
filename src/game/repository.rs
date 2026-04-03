use std::{cell::{Ref, RefCell}, rc::Rc};

use web_sys::Storage;
use crate::{game::SavedGameState, models::{GameResult}};

#[derive(Debug, Clone, PartialEq)]
pub struct Repository {
    items: Rc<RefCell<Vec<GameResult>>>,
    local_storage: Storage
}

impl Repository {
    pub fn new(local_storage: Storage) -> Self {

        let items = unsafe {
            let json = local_storage.get_item("records").unwrap_unchecked();
            let records: Option<Vec<_>> = json.and_then(|json| serde_json::from_str(&json).unwrap_unchecked());
            records.unwrap_or_default()
        };

        Self {
            items: Rc::new(RefCell::new(items)),
            local_storage
        }
    }

    pub fn clear_records(&self) {
        unsafe { self.local_storage.remove_item("records").unwrap_unchecked(); }
    }

    pub fn get_last_records(&self) -> Ref<'_, [GameResult]> {
        // self.items.sort_by_key(|r| r.created_on);
        let len = self.items.borrow().len();
        let start = len.saturating_sub(5);

        Ref::map(self.items.borrow(), |items| &items[start..])
    }

    pub fn set_last_record(&self, value: GameResult) {
        unsafe {
            self.items.borrow_mut().insert(0, value);

            let len = self.items.borrow().len();
            if self.items.borrow().len() > 5 {
                self.items.borrow_mut().remove(len - 1);
            }

            let json = serde_json::to_string(self.items.as_ref()).unwrap_unchecked();
            self.local_storage.set_item("records", &json).unwrap_unchecked();
        }
    }

    pub fn clear_game_session(&self) {
        unsafe { self.local_storage.remove_item("state").unwrap_unchecked(); }
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