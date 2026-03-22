use std::hint::unreachable_unchecked;

use chrono::{DateTime, Duration, Utc};
use gloo::console::info;
use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

pub trait SystemClock {
    
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum CellState {
    #[default]
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameCell {
    pub key: String,
    pub row_id: usize,
    pub column_id: usize,
    pub row: String,
    pub column: String,
    pub is_mine: bool,
    pub state: CellState,
    pub neighbor_mines: u8,
}

pub struct Grid(Vec<Vec<GameCell>>);

impl GameCell {
    fn new(row_id: usize, column_id: usize) -> Self {
        Self {
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
    pub cells: Vec<GameCell>,
    pub rows: usize,
    pub cols: usize,
    pub mines_count: usize,
    pub revealed_count: usize, 
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Idle,
    Initializing {
        cells: Vec<GameCell>,
        rows: usize,
        cols: usize,
        mines_count: usize,
        started_at: DateTime<Utc>,
    },
    Playing {
        cells: Vec<GameCell>,
        rows: usize,
        cols: usize,
        mines_count: usize,
        revealed_count: usize, 
        started_at: DateTime<Utc>,
    },
    GameOver {
        has_won: bool,
        cells: Vec<GameCell>,
        rows: usize,
        cols: usize,
        duration: Duration,
        started_at: DateTime<Utc>,
        mines_count: usize,
        revealed_count: usize, 
    }
}

pub struct GameSettings {
    pub rows: usize,
    pub cols: usize,
    pub mines_count: usize
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            rows: 15,
            cols: 15,
            mines_count: 2
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::Idle
    }

    pub fn game_over(has_won: bool) -> Self {
        
        let cells = Self::initialize_cells(15, 15);

        Self::GameOver {
            has_won,
            cells,
            rows: 15,
            cols: 15,
            duration: Duration::seconds(10),
            started_at: Utc::now(),
            mines_count: 5,
            revealed_count: 10
        }
    }

    pub fn play(&self, settings: GameSettings) -> Self {

        let GameSettings {
            rows,
            cols,
            mines_count
        } = settings;

        let cells = Self::initialize_cells(rows, cols);

        Self::Initializing {
            cells,
            rows,
            cols,
            mines_count,
            started_at: Utc::now()
        }
    }

    pub fn initialize_with_first_click(self, click_row: usize, click_col: usize) -> Self {
        match self {
            Self::Initializing { mut cells, rows, cols, mines_count, started_at } => {
                let first_idx = click_row * cols + click_col;
                
                Self::setup_mines(&mut cells, mines_count, first_idx);

                let mut revealed_count = 0;
     
                Self::calculate_neighbor_counts(&mut cells, rows, cols);
                Self::reveal_cell_internal(&mut cells, &mut revealed_count, rows, cols, click_row, click_col);
                
                Self::Playing {
                    cells,
                    rows,
                    cols,
                    mines_count,
                    revealed_count,
                    started_at,
                }
            },
            _ => self,
        }
    }

     fn reveal_cell_internal(
        cells: &mut [GameCell],
        revealed_count: &mut usize,
        rows: usize, 
        cols: usize, 
        row: usize, 
        col: usize
    ) {
        if !Self::in_bounds(row, col, rows, cols) {
            return;
        }

        let idx = Self::to_index(row, col, cols);

        if !Self::can_reveal(&cells[idx]) {
            return;
        }

        Self::reveal_single_cell(cells, revealed_count, idx);

        if Self::should_expand(&cells[idx]) {
            Self::reveal_neighbors(cells, revealed_count, rows, cols, row, col);
        }
    }

    fn in_bounds(row: usize, col: usize, rows: usize, cols: usize) -> bool {
        row < rows && col < cols
    }

    fn to_index(row: usize, col: usize, cols: usize) -> usize {
        row * cols + col
    }

    fn can_reveal(cell: &GameCell) -> bool {
         matches!(cell.state, CellState::Hidden)
    }

    fn should_expand(cell: &GameCell) -> bool {
        !cell.is_mine && cell.neighbor_mines == 0
    }

    fn reveal_single_cell(
        cells: &mut [GameCell],
        revealed_count: &mut usize,
        idx: usize,
    ) {
        cells[idx].state = CellState::Revealed;
        *revealed_count += 1;
    }

    fn reveal_neighbors(
        cells: &mut [GameCell],
        revealed_count: &mut usize,
        rows: usize,
        cols: usize,
        row: usize,
        col: usize,
    ) {
        for (nr, nc) in Self::neighbors(row, col, rows, cols) {
            let nidx = Self::to_index(nr, nc, cols);

            if matches!(cells[nidx].state, CellState::Revealed | CellState::Flagged) {
                continue;
            }

            Self::reveal_single_cell(cells, revealed_count, nidx);

            if cells[nidx].neighbor_mines == 0 && !cells[nidx].is_mine {
                Self::reveal_neighbors(cells, revealed_count, rows, cols, nr, nc);
            }
        }
    }

    fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(8);

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let nr = row as i32 + dr;
                let nc = col as i32 + dc;

                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    result.push((nr as usize, nc as usize));
                }
            }
        }

        result
    }

    pub fn toggle_flag(&self, row: usize, col: usize) -> Self {
        match self.clone() {
            Self::Playing {
                mut cells,
                rows,
                cols,
                mines_count,
                revealed_count,
                started_at,
            } => {
                let idx = row * cols + col;

                if idx >= cells.len() {
                    return Self::Playing {
                        cells,
                        rows,
                        cols,
                        mines_count,
                        revealed_count,
                        started_at,
                    };
                }

                cells[idx].state = match cells[idx].state {
                    CellState::Hidden => CellState::Flagged,
                    CellState::Flagged => CellState::Hidden,
                    CellState::Revealed => CellState::Revealed,
                };

                Self::Playing {
                    cells,
                    rows,
                    cols,
                    mines_count,
                    revealed_count,
                    started_at,
                }
            }
            _ => self.clone(),
        }
    }

    pub fn reveal(self, row: usize, col: usize) -> Self {
        match self {
            Self::Playing {
                mut cells,
                rows,
                cols,
                mines_count,
                mut revealed_count,
                started_at,
            } => {
                let idx = row * cols + col;

                if idx >= cells.len() || matches!(cells[idx].state, CellState::Flagged) {
                    return Self::Playing {
                        cells,
                        rows,
                        cols,
                        mines_count,
                        revealed_count,
                        started_at,
                    };
                }

                if cells[idx].is_mine {
                    cells[idx].state = CellState::Revealed;

                    return Self::GameOver {
                        has_won: false,
                        duration: Utc::now() - started_at,
                        cells,
                        rows,
                        cols,
                        started_at,
                        revealed_count: 0,
                        mines_count: 0
                    };
                }

                Self::reveal_cell_internal(
                    &mut cells,
                    &mut revealed_count,
                    rows,
                    cols,
                    row,
                    col,
                );

                let total_safe = rows * cols - mines_count;

                if revealed_count == total_safe {
                    return Self::GameOver {
                        has_won: true,
                        duration: Utc::now() - started_at,
                        cells,
                        rows,
                        cols,
                        started_at,
                        revealed_count: 0,
                        mines_count: 0
                    };
                }

                Self::Playing {
                    cells,
                    rows,
                    cols,
                    mines_count,
                    revealed_count,
                    started_at,
                }
            }
            _ => self,
        }
    }

    fn initialize_cells(rows: usize, cols: usize,) -> Vec<GameCell> {
        let total_cells = rows * cols;
        let mut cells = Vec::with_capacity(total_cells);

        for row in 0..rows {
            for col in 0..cols {
                cells.push(GameCell::new(row, col));
            }
        }

        cells
    }

    fn setup_mines(cells: &mut [GameCell], mines_count: usize, forbidden_idx: usize) {
        let mut indices: Vec<usize> = (0..cells.len())
            .filter(|&i| i != forbidden_idx)
            .collect();

        let mut rng = rng();
        indices.shuffle(&mut rng);

        for &idx in indices.iter().take(mines_count) {
            cells[idx].is_mine = true;
        }
    }

    fn calculate_neighbor_counts(cells: &mut [GameCell], rows: usize, cols: usize) {
        for row in 0..rows {
            for col in 0..cols {
                let idx = row * cols + col;
                if cells[idx].is_mine {
                    continue;
                }

                cells[idx].neighbor_mines = Self::count_adjacent_mines(cells, row, col, rows, cols);
            }
        }
    }
    
    fn count_adjacent_mines(cells: &[GameCell], row: usize, col: usize, rows: usize, cols: usize) -> u8 {
        let mut count = 0;
        
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 { continue; }
                
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;
                
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let neighbor_idx = (nr as usize) * cols + (nc as usize);
                    if cells[neighbor_idx].is_mine {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    fn to_state(self) -> SavedGameState {

        match self {
            GameState::GameOver {
                has_won,
                cells,
                rows,
                cols,
                duration,
                mines_count,
                revealed_count,
                started_at } => {
                SavedGameState {
                    cells,
                    rows,
                    cols,
                    mines_count,
                    revealed_count,
                    started_at
                }
            },
            _ => unsafe { unreachable_unchecked() }
        }
    }
}