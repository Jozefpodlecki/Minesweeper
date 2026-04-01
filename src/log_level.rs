use log::Level;
use web_sys::Storage;

pub struct LocalStorageLogLevel(Storage);

impl LocalStorageLogLevel {
    pub fn new(storage: Storage) -> Self {
        Self(storage)
    }

    pub fn default() -> Level {
        if cfg!(debug_assertions) {
            Level::Debug
        } else {
            Level::Error
        }
    }

    pub fn set_default_if_missing(&self) {
        if self.get().is_none() {
            self.set(Self::default());    
        }
    }

    pub fn get_default(&self) -> Level {
        self.get().unwrap_or_else(|| Self::default())
    }

    pub fn get(&self) -> Option<Level> {
        self.0
            .get_item("RUST_LOG")
            .ok()
            .flatten()
            .as_deref()
            .map(Self::str_to_level)
    }

    pub fn set(&self, value: Level) {
        let json = match value {
            Level::Error => "error",
            Level::Warn => "warn",
            Level::Info => "info",
            Level::Debug => "debug",
            Level::Trace => "trace",
        };

        unsafe { self.0.set_item("RUST_LOG", json).unwrap_unchecked(); }
    }

    fn str_to_level(value: &str) -> Level {
        match value {
            "error" => Level::Error,
            "warn" => Level::Warn,
            "info" => Level::Info,
            "debug" => Level::Debug,
            "trace" => Level::Trace,
            _ => Level::Error
        }
    }

    fn level_to_str(value: Level) -> &'static str {
        match value {
            Level::Error => "error",
            Level::Warn => "warn",
            Level::Info => "info",
            Level::Debug => "debug",
            Level::Trace => "trace",
        }
    }
}
