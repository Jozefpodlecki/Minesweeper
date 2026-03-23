use std::hint::unreachable_unchecked;

use chrono::{DateTime, Duration, Utc};
use gloo::console::info;
use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum CellState {
    #[default]
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameCell {
    pub id: usize,
    pub key: String,
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
    pub fn new_with_state(row_id: usize, column_id: usize, columns: usize, is_mine: bool, state: CellState) -> Self {
        Self {
            id: row_id * columns + column_id,
            key: format!("{}-{}", row_id, column_id),
            row_id,
            column_id,
            row: row_id.to_string(),
            column: column_id.to_string(),
            is_mine,
            state,
            neighbor_mines: 0,
        }
    }

    pub fn new(row_id: usize, column_id: usize, columns: usize) -> Self {
        Self {
            id: row_id * columns + column_id,
            key: format!("{}-{}", row_id, column_id),
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

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            rows: 15,
            columns: 15,
            mines_count: 2
        }
    }
}

impl GameSettings {
    pub fn hard() -> Self {
        Self {
            rows: 15,
            columns: 15,
            mines_count: 50
        }
    }
}
