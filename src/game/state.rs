use std::hint::unreachable_unchecked;
use log::*;
use chrono::{Duration, Utc};

use crate::{
    game::{
        logic,
        model::{CellState, GameCell, SavedGameState},
        phase::GamePhase, GameSettings,
    }, models::GameResult, services::{DefaultSystemClock, SystemClock}
};

#[derive(Debug, Clone, PartialEq)]
pub struct GameState<SC: SystemClock> {
    clock: SC,
    phase: GamePhase,
}

impl<SC: Clone + SystemClock> GameState<SC> {
    pub fn new(clock: SC) -> Self {
        Self {
            clock,
            phase: GamePhase::default(),
        }
    }

    /// Start a new game
    pub fn play(&self, settings: GameSettings) -> Self {
        let GameSettings {
            rows,
            columns,
            mines_count,
        } = settings;

        let cells = logic::initialize_cells(rows, columns);

        Self {
            clock: self.clock.clone(),
            phase: GamePhase::Initializing {
                cells,
                rows,
                columns,
                mines_count,
            },
        }
    }

    pub fn restart(&self) -> Self {
        let phase = match self.phase.clone() {
            GamePhase::GameOver { mut cells, rows, columns, mines_count, .. } => {

                for cell in &mut cells {
                    cell.neighbor_mines = 0;
                    cell.state = CellState::Hidden;
                    cell.is_mine = false;
                }

                GamePhase::Initializing {
                    cells,
                    rows,
                    columns,
                    mines_count,
                }
            },
            _=> unsafe { unreachable_unchecked() }
        };

        Self {
            clock: self.clock.clone(),
            phase,
        }
    }

    pub fn phase(&self) -> &GamePhase {
        &self.phase
    }

    pub fn neighbors<'a>(
        &self,
        cells: &'a [GameCell],
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<&'a GameCell> {
        let mut result = Vec::with_capacity(8);

        for dr in [-1isize, 0, 1] {
            for dc in [-1isize, 0, 1] {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let nr = row as isize + dr;
                let nc = col as isize + dc;

                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                    let idx = nr as usize * cols + nc as usize;
                    result.push(&cells[idx]);
                }
            }
        }

        result
    }

    fn mines_left(cells: &[GameCell], total_mines: usize) -> isize {
        let flagged = cells
            .iter()
            .filter(|c| matches!(c.state, CellState::Flagged))
            .count();

        total_mines as isize - flagged as isize
    }

    pub fn toggle_flag(&self, row: usize, col: usize) -> Self {
        let phase = match &self.phase {
            GamePhase::Playing {
                cells,
                rows,
                columns,
                mines_count,
                revealed_count,
                flags_count,
                started_at,
            } => {
                let mut flags_count = *flags_count;
                let mut cells = cells.clone();
                let idx = row * *columns + col;

                if idx < cells.len() {
                    cells[idx].state = match cells[idx].state {
                        CellState::Hidden => {
                            flags_count += 1;
                            CellState::Flagged
                        },
                        CellState::Flagged => {
                            flags_count -= 1;
                            CellState::Hidden
                        },
                        CellState::Revealed => CellState::Revealed,
                    };
                }

                GamePhase::Playing {
                    cells,
                    rows: *rows,
                    columns: *columns,
                    mines_count: *mines_count,
                    revealed_count: *revealed_count,
                    flags_count,
                    started_at: *started_at,
                }
            }
            other => other.clone(),
        };

        Self {
            clock: self.clock.clone(),
            phase,
        }
    }

    fn reveal_all_cells(cells: &mut [GameCell]) {
        for cell in cells.iter_mut() {
            cell.state = CellState::Revealed;
        }
    }

    /// Reveal a cell (core gameplay action)
    pub fn reveal(self, row: usize, col: usize) -> Self {
        let phase = match self.phase {
            GamePhase::Initializing {
                mut cells,
                rows,
                columns,
                mines_count,
            } => {
                let first_idx = row * columns + col;

                let ids = logic::get_forbidden_indices(row, col, rows, columns);
                logic::setup_mines(&mut cells, mines_count, &ids);
                logic::calculate_neighbor_counts(&mut cells, rows, columns);

                let mut revealed_count = 0;

                logic::reveal_cell(
                    &mut cells,
                    &mut revealed_count,
                    row,
                    col,
                    rows,
                    columns,
                );

                GamePhase::Playing {
                    cells,
                    rows,
                    columns,
                    mines_count,
                    revealed_count,
                    flags_count: 0,
                    started_at: self.clock.utc_now(),
                }
            },
            GamePhase::Playing {
                mut cells,
                rows,
                columns,
                mines_count,
                mut revealed_count,
                flags_count,
                started_at,
            } => {
                let idx = row * columns + col;

                if idx >= cells.len()
                    || matches!(cells[idx].state, CellState::Flagged)
                {
                    return Self {
                        clock: self.clock.clone(),
                        phase: GamePhase::Playing {
                            cells,
                            rows,
                            columns,
                            mines_count,
                            revealed_count,
                            flags_count,
                            started_at,
                        },
                    };
                }

                if cells[idx].is_mine {
                    cells[idx].state = CellState::Revealed;
 
                    Self::reveal_all_cells(&mut cells);

                    return Self {
                        clock: self.clock.clone(),
                        phase: GamePhase::GameOver {
                            has_won: false,
                            duration: self.clock.utc_now() - started_at,
                            cells,
                            rows,
                            columns,
                            started_at,
                            revealed_count,
                            mines_count,
                            flags_count
                        },
                    };
                }

                logic::reveal_cell(
                    &mut cells,
                    &mut revealed_count,
                    row,
                    col,
                    rows,
                    columns,
                );

                let total_safe = rows * columns - mines_count;

                if revealed_count == total_safe {
                    Self::reveal_all_cells(&mut cells);

                    return Self {
                        clock: self.clock.clone(),
                        phase: GamePhase::GameOver {
                            has_won: true,
                            duration: self.clock.utc_now() - started_at,
                            cells,
                            rows,
                            columns,
                            started_at,
                            revealed_count,
                            mines_count,
                            flags_count
                        },
                    };
                }

                GamePhase::Playing {
                    cells,
                    rows,
                    columns,
                    mines_count,
                    revealed_count,
                    flags_count,
                    started_at,
                }
            }
            other => other,
        };

        Self {
            clock: self.clock.clone(),
            phase,
        }
    }

    pub fn to_result(&self) -> GameResult {
        match self.phase.clone() {
            GamePhase::GameOver {
                has_won,
                started_at,
                rows,
                columns,
                mines_count,
                revealed_count,
                flags_count,
                duration,
                ..
            } => GameResult {
                rows,
                columns,
                has_won,
                started_at,
                duration,
                revealed_count,
                mines_count,
                flags_count
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }

    /// Convert finished game into a saved state
    pub fn to_state(&self) -> SavedGameState {
        match self.phase.clone() {
            GamePhase::Playing {
                cells,
                rows,
                columns,
                mines_count,
                revealed_count,
                started_at,
                ..
            } => SavedGameState {
                cells,
                rows,
                columns,
                mines_count,
                revealed_count,
                started_at,
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl GameState<DefaultSystemClock> {
    pub fn default() -> Self {
        Self {
            clock: DefaultSystemClock::default(),
            phase: GamePhase::default(),
        }
    }

    pub fn initializing(rows: usize, columns: usize) -> Self {
        let clock = DefaultSystemClock::default();
        let cells = logic::initialize_cells(rows, columns);

        Self {
            clock,
            phase: GamePhase::Initializing {
                cells,
                rows,
                columns,
                mines_count: 0
            },
        }
    }

    pub fn playing(cells: Vec<GameCell>, rows: usize, columns: usize) -> Self {
        let clock = DefaultSystemClock::default();
        let started_at = clock.utc_now();

        Self {
            clock,
            phase: GamePhase::Playing {
                cells: cells.into(),
                rows,
                columns,
                mines_count: 0,
                revealed_count: 0,
                flags_count: 0,
                started_at
            },
        }
    }

    pub fn game_over(has_won: bool) -> Self {
        let cells = logic::initialize_cells(15, 15);

        Self {
            clock: DefaultSystemClock::default(),
            phase: GamePhase::GameOver {
                has_won,
                cells,
                rows: 15,
                columns: 15,
                duration: Duration::seconds(10),
                started_at: Utc::now(),
                mines_count: 5,
                revealed_count: 10,
                flags_count: 0
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{GamePhase, GameCell, CellState};

    #[test]
    fn should() {
        
    }
}