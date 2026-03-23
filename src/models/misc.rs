use std::{error::Error, fmt::{self, Debug, Display, Formatter}, str::FromStr};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use crate::models::AppError;

#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    Loading,
    Error(AppError),
    Loaded(Social)
}

impl Default for AppState {
    fn default() -> Self {
        Self::Loading
    }
}
    

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Social {
    pub linkedin: String,
    pub github: String,
    pub portfolio: String
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

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameDifficulty {
    #[default]
    Easy,
    Medium,
    Hard
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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
pub struct Settings {
    pub difficulty: GameDifficulty,
    pub engine: GameEngine,
    pub background_url: Option<String>,
    pub persist_game: bool
}