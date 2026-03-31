use yew::UseStateHandle;

use crate::{models::Settings, services::StorageAccessor};

#[derive(Debug, Clone)]
pub struct SettingsManager {
    state: UseStateHandle<Settings>,
    storage: StorageAccessor<Settings>
}

impl PartialEq for SettingsManager {
    fn eq(&self, value: &Self) -> bool {
        self.state == value.state
    }
}

impl SettingsManager {
    pub fn new(state: UseStateHandle<Settings>, storage: StorageAccessor<Settings>) -> Self {
        Self { state, storage }
    }

    pub fn init(&self) {
        if self.storage.get().is_none() {
            let default = &*self.state;
            self.storage.set(default);
        }
    }

    pub fn get(&self) -> Settings {
        self.storage.get().unwrap_or_default()
    }

    pub fn save(&self, value: Settings) {
        self.storage.set(&value);
        self.state.set(value);
    }
}