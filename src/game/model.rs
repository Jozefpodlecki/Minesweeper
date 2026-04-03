use std::{fmt, rc::Rc};

use chrono::{DateTime, Duration, Utc};
use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::models::GameDifficulty;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct GameCell {
    pub id: usize,
    pub key: Box<str>,
    pub row_id: usize,
    pub column_id: usize,
    pub row: Rc<str>,
    pub column: Rc<str>,
    pub is_mine: bool,
    pub state: CellState,
    pub neighbor_mines: usize,
}

impl GameCell {
    pub fn from_saved(value: SavedGameCell) -> Self {
        Self {
            id: value.id,
            key: format!("{}-{}", value.row_id, value.column_id).into(),
            row_id: value.row_id,
            column_id: value.column_id,
            row: value.row_id.to_string().into(),
            column: value.column_id.to_string().into(),
            is_mine: value.is_mine,
            state: value.state,
            neighbor_mines: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state = CellState::Hidden;
        self.is_mine = false;
        self.neighbor_mines = 0;
    }
}

impl fmt::Display for GameCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cell[id={}, row={}, col={}, mine={}, state={:?}, neighbors={}]",
            self.id,
            self.row,
            self.column,
            self.is_mine,
            self.state,
            self.neighbor_mines
        )
    }
}

impl GameCell {
    #[cfg(test)]
    pub fn test(row_id: usize, column_id: usize, columns: usize, is_mine: bool, neighbor_mines: usize, state: CellState) -> Self {
        Self {
            id: row_id * columns + column_id,
            key: format!("{}-{}", row_id, column_id).into(),
            row_id,
            column_id,
            row: row_id.to_string().into(),
            column: column_id.to_string().into(),
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
            row: row_id.to_string().into(),
            column: column_id.to_string().into(),
            is_mine: false,
            state: Default::default(),
            neighbor_mines: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGameCell {
    pub id: usize,
    pub row_id: usize,
    pub column_id: usize,
    pub is_mine: bool,
    pub state: CellState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGameGrid {
    pub cells: Box<[SavedGameCell]>,
    pub rows: usize,
    pub columns: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGameState {
    pub settings: GameSettings,
    pub grid: SavedGameGrid,
    pub mines_count: usize,
    pub revealed_count: usize, 
    pub flags_count: usize,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    pub rows: usize,
    pub columns: usize,
    pub mines_count: usize
}