use wasm_bindgen::JsError;
use web_sys::{window, Storage};

use crate::{game::SavedGameState, models::Record};


#[derive(Debug, Clone, PartialEq)]
pub struct Repository(Storage);

impl Repository {
    pub fn new(local_storage: Storage) -> Self {
        Self(local_storage)
    }

    pub fn get_last_records(&self) -> Vec<Record> {
        unsafe {
            let json = self.0.get_item("records").unwrap_unchecked();
            let records = json.and_then(|json| serde_json::from_str(&json).unwrap_unchecked());
            records.unwrap_or_default()
        }
    }

    pub fn set_last_record(&self, value: Record) -> Result<(), JsError> {
        Ok(())
    }

    pub fn get_last_state(&self) -> Option<SavedGameState> {
        None
        // let records = self.0.get_item(key);
        // ::get::<SavedGameState>("state");

        // records.ok()
        //  let key = key.as_ref();
        // let item = Self::raw()
        //     .get_item(key)
        //     .expect_throw("unreachable: get_item does not throw an exception")
        //     .ok_or_else(|| StorageError::KeyNotFound(key.to_string()))?;
        // let item = serde_json::from_str(&item)?;
        // Ok(item)
    }
}



// pub fn get_last_state() -> Option<SavedGameState> {
//     let records = LocalStorage::get::<SavedGameState>("state");

//     records.ok()
// }

// pub fn save_state(value: SavedGameState) -> Result<(), JsError> {
//     LocalStorage::set("state", value)
//         .map_err(|err| {
//             let js_error = js_sys::Error::new(&err.to_string());
//             JsError::from(js_error)
//         })?;

//     Ok(())
// }

// pub fn get_last_records() -> Vec<Record> {
//     let records = LocalStorage::get::<Vec<Record>>("records");

//     records.unwrap_or_default()
// }

// pub fn set_last_record(value: Record) -> Result<(), JsError> {
//     let records = LocalStorage::get::<Vec<Record>>("records");
//     let mut records = records.unwrap_or_default();

//     records.push(value);

//     LocalStorage::set("records", records)
//         .map_err(|err| {
//             let js_error = js_sys::Error::new(&err.to_string());
//             JsError::from(js_error)
//         })?;

//     Ok(())
// }
