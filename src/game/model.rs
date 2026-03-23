use chrono::{DateTime, Duration, Utc};
use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::models::GameDifficulty;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum CellState {
    #[default]
    Hidden,
    Revealed,
    Flagged,
}

pub struct DisplayValue {
    pub raw: usize,
    pub formatted: Box<str>
}

impl DisplayValue {
    pub fn new(raw: usize) -> Self {
        Self {
            raw,
            formatted: raw.to_string().into()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameCell {
    pub id: usize,
    pub key: Box<str>,
    pub row_id: usize,
    pub column_id: usize,
    pub row: String,
    pub column: String,
    pub is_mine: bool,
    pub state: CellState,
    pub neighbor_mines: usize,
}


impl GameCell {
    #[cfg(test)]
    pub fn test(row_id: usize, column_id: usize, columns: usize, is_mine: bool, neighbor_mines: usize, state: CellState) -> Self {
        Self {
            id: row_id * columns + column_id,
            key: format!("{}-{}", row_id, column_id).into(),
            row_id,
            column_id,
            row: row_id.to_string(),
            column: column_id.to_string(),
            is_mine,
            state,
            neighbor_mines,
        }
    }

    pub fn new(row_id: usize, column_id: usize, columns: usize) -> Self {
        Self {
            id: row_id * columns + column_id,
            key: format!("{}-{}", row_id, column_id).into(),
            row_id,
            column_id,
            row: row_id.to_string(),
            column: column_id.to_string(),
            is_mine: false,
            state: Default::default(),
            neighbor_mines: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGameState {
    pub cells: Box<[GameCell]>,
    pub rows: usize,
    pub columns: usize,
    pub mines_count: usize,
    pub revealed_count: usize, 
    pub started_at: DateTime<Utc>,
}

pub struct GameSettings {
    pub rows: usize,
    pub columns: usize,
    pub mines_count: usize
}

impl GameSettings {
    pub fn from_difficulty(rows: usize, columns: usize, difficulty: GameDifficulty) -> Self {
        let mut rng = rand::rng();

        let density = match difficulty {
            GameDifficulty::Easy => rng.random_range(0.10..0.14),
            GameDifficulty::Medium => rng.random_range(0.15..0.19),
            GameDifficulty::Hard => rng.random_range(0.24..0.29),
        };

        let mines_count = (rows * columns) as f64 * density;

        Self { rows, columns, mines_count: mines_count as usize }
    }
}
