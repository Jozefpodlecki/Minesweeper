use std::hint::unreachable_unchecked;
use chrono::{Duration, Utc};
use rand::RngExt;
use yew::UseStateHandle;
use crate::{
    game::{logic, model::{CellState, GameCell, SavedGameState}, phase::GamePhase, GameGrid, GameSettings},
    models::{GameDifficulty, GameResult, Settings},
    services::{DefaultSystemClock, SystemClock},
};

pub type DefaultGameManager = GameManager<DefaultSystemClock>;

    // pub fn from_difficulty(rows: usize, columns: usize, difficulty: GameDifficulty) -> Self {
    //     let mut rng = rand::rng();

    //     let density = match difficulty {
    //         GameDifficulty::Easy => rng.random_range(0.10..0.14),
    //         GameDifficulty::Medium => rng.random_range(0.15..0.19),
    //         GameDifficulty::Hard => rng.random_range(0.24..0.29),
    //     };

    //     let mines_count = (rows * columns) as f64 * density;

    //     Self { rows, columns, mines_count: mines_count as usize }
    // }

#[derive(Clone)]
pub struct GameManager<SC: SystemClock> {
    clock: SC,
    settings: UseStateHandle<Settings>
}

impl<SC: SystemClock> PartialEq for GameManager<SC> {
    fn eq(&self, value: &Self) -> bool {
        self.settings == value.settings
    }
}

impl<SC: Clone + SystemClock> GameManager<SC> {
    pub fn new(clock: SC, settings: UseStateHandle<Settings>) -> Self {
        Self { clock, settings  }
    }

    pub fn create(&self) -> GameState<SC> {
        let settings = &*self.settings;

        let rows = 15;
        let columns = 15;
        let mut rng = rand::rng();

        let density = match settings.difficulty {
            GameDifficulty::Easy => rng.random_range(0.10..0.14),
            GameDifficulty::Medium => rng.random_range(0.15..0.19),
            GameDifficulty::Hard => rng.random_range(0.24..0.29),
        };

        let mines_count = (rows * columns) as f64 * density;

        let settings = GameSettings {
            rows,
            columns,
            mines_count: mines_count as usize
        };

        GameState::new(self.clock.clone(), settings)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameState<SC: SystemClock> {
    settings: GameSettings,
    clock: SC,
    phase: GamePhase,
}

impl<SC: Clone + SystemClock> GameState<SC> {

    pub fn test(clock: SC, settings: GameSettings, phase: GamePhase) -> Self {
        Self {
            clock,
            settings,
            phase
        }
    }

    pub fn new(clock: SC, settings: GameSettings) -> Self {
        Self { clock, settings, phase: GamePhase::default() }
    }

    /// Start a new game
    pub fn play(&self) -> Self {
        let GameSettings { rows, columns, mines_count } = self.settings;
        let grid = GameGrid::new(rows, columns);

        Self {
            clock: self.clock.clone(),
            settings: self.settings.clone(),
            phase: GamePhase::Initializing { grid, mines_count },
        }
    }

    /// Restart after game over
    pub fn restart(&self) -> Self {
        match &self.phase {
            GamePhase::GameOver { grid, mines_count, .. } => {
                let mut new_grid = grid.clone();
                new_grid.reset();

                Self {
                    clock: self.clock.clone(),
                    settings: self.settings.clone(),
                    phase: GamePhase::Initializing { grid: new_grid, mines_count: *mines_count },
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn phase(&self) -> &GamePhase {
        &self.phase
    }

    pub fn toggle_flag(&self, row: usize, col: usize) -> Self {
        let phase = match &self.phase {
            GamePhase::Playing { grid, mines_count, revealed_count, flags_count, started_at } => {
                let mut new_grid = grid.clone();
                let mut new_flags = *flags_count;
                match new_grid.toggle_flag(row, col) {
                    CellState::Flagged => new_flags += 1,
                    CellState::Hidden => new_flags -= 1,
                    _ => {}
                }

                GamePhase::Playing {
                    grid: new_grid,
                    mines_count: *mines_count,
                    revealed_count: *revealed_count,
                    flags_count: new_flags,
                    started_at: *started_at,
                }
            }
            other => other.clone(),
        };

        Self {
            clock: self.clock.clone(),
            settings: self.settings.clone(),
            phase
        }
    }

    /// Reveal a cell
    pub fn reveal(self, row: usize, col: usize) -> Self {
        let phase = match self.phase {
            GamePhase::Initializing { mut grid, mines_count } => {
                let forbidden_indices: Vec<_> = grid.forbidden_indices(row, col).collect();
                grid.setup_mines(mines_count, forbidden_indices);
                grid.calculate_neighbor_counts();

                let revealed_count = grid.reveal_cell(row, col);

                GamePhase::Playing {
                    grid,
                    mines_count,
                    revealed_count,
                    flags_count: 0,
                    started_at: self.clock.utc_now(),
                }
            }
            GamePhase::Playing { mut grid, mines_count, mut revealed_count, flags_count, started_at } => {
                let idx = grid.index(row, col);
                // if idx >= grid.len() || matches!(grid.cells[idx].state, CellState::Flagged) {
                //     return Self { clock: self.clock.clone(), phase: GamePhase::Playing { grid, mines_count, revealed_count, flags_count, started_at } };
                // }

                if grid.is_mine(idx) {
                    grid.reveal_all();
                    let last_cell = Some(grid.at(row, col).clone());
                    return Self {
                        clock: self.clock.clone(),
                        settings: self.settings.clone(),
                        phase: GamePhase::GameOver {
                            has_won: false,
                            grid,
                            duration: self.clock.utc_now() - started_at,
                            started_at,
                            mines_count,
                            revealed_count,
                            flags_count,
                            last_cell
                        },
                    };
                }

                let revealed_count = grid.reveal_cell(row, col);
                let total_safe = grid.rows * grid.columns - mines_count;

                if revealed_count == total_safe {
                    grid.reveal_all();
                    return Self {
                        clock: self.clock.clone(),
                        settings: self.settings.clone(),
                        phase: GamePhase::GameOver {
                            has_won: true,
                            grid,
                            duration: self.clock.utc_now() - started_at,
                            started_at,
                            mines_count,
                            revealed_count,
                            flags_count,
                            last_cell: None
                        },
                    };
                }

                GamePhase::Playing { grid, mines_count, revealed_count, flags_count, started_at }
            }
            other => other,
        };

        Self {
            clock: self.clock.clone(),
            settings: self.settings.clone(),
            phase
        }
    }

    pub fn to_result(&self) -> GameResult {
        match &self.phase {
            GamePhase::GameOver {
                has_won,
                grid,
                started_at,
                duration,
                mines_count,
                revealed_count,
                flags_count,
                last_cell
            } => {
                GameResult {
                    rows: grid.rows,
                    columns: grid.columns,
                    has_won: *has_won,
                    started_at: *started_at,
                    duration: *duration,
                    revealed_count: *revealed_count,
                    mines_count: *mines_count,
                    flags_count: *flags_count,
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn to_state(&self) -> SavedGameState {
        match &self.phase {
            GamePhase::Playing { grid, mines_count, revealed_count, started_at, .. } => SavedGameState {
                cells: grid.cells().clone().into(),
                rows: grid.rows,
                columns: grid.columns,
                mines_count: *mines_count,
                revealed_count: *revealed_count,
                started_at: *started_at,
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}