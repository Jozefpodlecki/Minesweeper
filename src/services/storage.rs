use web_sys::Storage;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct StorageAccessor<T> {
    key: String,
    storage: Storage,
    _marker: std::marker::PhantomData<T>,
}

impl<T> StorageAccessor<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(key: impl Into<String>, storage: Storage) -> Self {
        Self {
            key: key.into(),
            storage,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get(&self) -> Option<T> {
        self.storage
            .get_item(&self.key)
            .ok()
            .flatten()
            .and_then(|s| serde_json::from_str(&s).ok())
    }

    pub fn set(&self, value: &T) {
        if let Ok(json) = serde_json::to_string(value) {
            let _ = self.storage.set_item(&self.key, &json);
        }
    }

    #[allow(unused)]
    pub fn remove(&self) {
        unsafe { self.storage.remove_item(&self.key).unwrap_unchecked(); }
    }
}