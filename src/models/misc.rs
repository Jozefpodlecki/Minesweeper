use std::{error::Error, fmt::{self, Debug, Display, Formatter}, hint::unreachable_unchecked, str::FromStr};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use crate::models::AppError;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum AppState {
    #[default]
    Loading,
    Error(AppError),
    Loaded(Social)
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Social {
    pub linkedin: Box<str>,
    pub github: Box<str>,
    pub portfolio: Box<str>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameResult {
    pub has_won: bool,
    pub started_at: DateTime<Utc>,
    pub duration: Duration,
    pub rows: usize,
    pub columns: usize,
    pub revealed_count: usize,
    pub mines_count: usize,
    pub flags_count: usize
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameDifficulty {
    #[default]
    Easy,
    Medium,
    Hard
}

impl GameDifficulty {
    pub fn label(&self) -> &'static str {
        match self {
            GameDifficulty::Easy => "Easy",
            GameDifficulty::Medium => "Medium",
            GameDifficulty::Hard => "Hard",
        }
    }

    pub fn all() -> &'static [GameDifficulty] {
        &[
            GameDifficulty::Easy,
            GameDifficulty::Medium,
            GameDifficulty::Hard,
        ]
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameEngine {
    #[default]
    Html,
    Canvas
}

impl FromStr for GameDifficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "easy" => Ok(GameDifficulty::Easy),
            "medium" => Ok(GameDifficulty::Medium),
            "hard" => Ok(GameDifficulty::Hard),
            _ => Err(()),
        }
    }
}

impl fmt::Display for GameDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameDifficulty::Easy => write!(f, "easy"),
            GameDifficulty::Medium => write!(f, "medium"),
            GameDifficulty::Hard => write!(f, "hard"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackgroundSource {
    #[default]
    Default,
    FileSystem {
        uploaded_on: DateTime<Utc>,
        file_name: Box<str>,
        data_url: Box<str>
    },
    Url {
        uploaded_on: DateTime<Utc>,
        url: Box<str>,
        data_url: Box<str>
    }
}

// impl PartialEq for BackgroundSource {
//     fn eq(&self, other: &Self) -> bool {
//         let verdict = match (self, other) {
//             (Self::FileSystem { uploaded_on: l_uploaded_on, file_name: l_file_name, data_url: l_data_url }, Self::FileSystem { uploaded_on: r_uploaded_on, file_name: r_file_name, data_url: r_data_url }) => l_uploaded_on == r_uploaded_on && l_file_name == r_file_name && l_data_url == r_data_url,
//             (Self::Url { uploaded_on: l_uploaded_on, url: l_url, data_url: l_data_url }, Self::Url { uploaded_on: r_uploaded_on, url: r_url, data_url: r_data_url }) => l_uploaded_on == r_uploaded_on && l_url == r_url && l_data_url == r_data_url,
//             _ => core::mem::discriminant(self) == core::mem::discriminant(other),
//         };
//         log::info!("{self} eq {other} = {verdict}");
//         verdict
//     }
// }

impl fmt::Display for BackgroundSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackgroundSource::Default => write!(f, "Default background"),
            BackgroundSource::FileSystem { uploaded_on, file_name, data_url } => {
                write!(
                    f,
                    "FileSystem: {} (uploaded on {}) [data_url: {}]",
                    file_name, uploaded_on, data_url.get(0..30).unwrap_or_else(|| "None")
                )
            },
            BackgroundSource::Url { uploaded_on, url, data_url } => {
                write!(
                    f,
                    "Url: {} (uploaded on {}) [data_url: {}]",
                    url, uploaded_on, data_url.get(0..30).unwrap_or_else(|| "None")
                )
            }
        }
    }
}

impl BackgroundSource {
    pub fn default_url() -> Self {
        Self::Url {
            uploaded_on: Default::default(),
            url: Default::default(),
            data_url: Default::default()
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "default" => Self::Default,
            "file" => Self::FileSystem {
                uploaded_on: Default::default(),
                file_name: "".into(),
                data_url: "".into(),
            },
            "url" => Self::Url {
                uploaded_on: Default::default(),
                url: "".into(),
                data_url: "".into(),
            },
            _ => Self::Default,
        }
    }

    pub fn clear(&mut self) {
        match self {
            BackgroundSource::FileSystem { uploaded_on, file_name, data_url } => {
                *uploaded_on = Default::default();
                *file_name = Default::default();
                *data_url = Default::default();
            },
            _ => {}
        }
    }

    pub fn data_url(&self) -> Option<&str> {
        match self {
            Self::Url { data_url, .. } => {
                Some(&data_url)
            }
            Self::FileSystem { data_url, .. } => {
                Some(data_url)
            }
            _ => None
        }
    }

    pub fn set_data_url(&mut self, value: &str) {
        match self {
            Self::Url { data_url, .. } => {
                *data_url = value.into();
            }
            BackgroundSource::FileSystem { data_url, .. } => {
                *data_url = value.into();
            }
            _ => unsafe { unreachable_unchecked() }
        };
    }

    pub fn set_url(&mut self, value: &str) {
        match self {
            Self::Url { url, .. } => {
                *url = value.into();
            }
            _ => unsafe { unreachable_unchecked() }
        };
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::FileSystem { .. } => "file",
            Self::Url { .. } => "url",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub difficulty: GameDifficulty,
    pub engine: GameEngine,
    pub background: BackgroundSource,
    pub persist_game: bool
}