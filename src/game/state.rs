use chrono::{DateTime, Duration, Utc};
use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameCell {
    pub row_id: usize,
    pub column_id: usize,
    pub is_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub neighbor_mines: u8,
}

pub struct Grid(Vec<Vec<GameCell>>);

impl GameCell {
    fn new(row_id: usize, column_id: usize) -> Self {
        Self {
            row_id,
            column_id,
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
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
            rows: 10,
            cols: 10,
            mines_count: 5
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::Idle
    }

    pub fn play(self, settings: GameSettings) -> Self {

        let GameSettings {
            rows,
            cols,
            mines_count
        } = settings;

        let mut cells = Self::initialize_cells(rows, cols);

        Self::Playing {
            cells,
            rows,
            cols,
            mines_count,
            revealed_count: 0,
            started_at: Utc::now()
        }
    }

    pub fn initialize_with_first_click(self, click_row: usize, click_col: usize) -> Self {
        match self {
            Self::Playing { mut cells, rows, cols, mines_count, mut revealed_count, started_at } => {
                let first_idx = click_row * cols + click_col;
                
                let mut indices: Vec<usize> = (0..cells.len())
                    .filter(|&i| i != first_idx)
                    .collect();

                let mut rng = rng();
                indices.shuffle(&mut rng);
                
                for &idx in indices.iter().take(mines_count) {
                    cells[idx].is_mine = true;
                }
     
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
        cells: &mut Vec<GameCell>, 
        revealed_count: &mut usize,
        rows: usize, 
        cols: usize, 
        row: usize, 
        col: usize
    ) {
        let idx = row * cols + col;
        if idx >= cells.len() { return; }
        
        if !cells[idx].is_revealed && !cells[idx].is_flagged {
            cells[idx].is_revealed = true;
            *revealed_count += 1;
            
            if cells[idx].is_mine {
            } else if cells[idx].neighbor_mines == 0 {

            }
        }
    }

    fn setup_cells(
        rows: usize,
        cols: usize,
        mines_count: usize) -> Vec<GameCell> {
       
        let mut cells = Self::initialize_cells(rows, cols);
        Self::setup_mines(&mut cells, mines_count);
        Self::calculate_neighbor_counts(&mut cells, rows, cols);

        cells
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

    fn setup_mines(cells: &mut [GameCell], mines_count: usize) {
        let total_cells = cells.len();
        let mut rng = rng();
        let mut indices: Vec<usize> = (0..total_cells).collect();
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
}