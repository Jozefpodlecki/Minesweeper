use rand::seq::IndexedRandom;

use crate::{game::{CellState, GameCell, GamePhase, GameState}, services::SystemClock};

pub enum AiAction {
    None,
    Reveal {
        col: usize,
        row: usize
    },
    Flag {
        col: usize,
        row: usize
    },
    Restart
}

pub struct AiAigent;

impl AiAigent {
    pub fn new() -> Self {
        Self{}
    }

    pub fn next<SC: SystemClock + Clone>(&self, state: &GameState<SC>) -> AiAction {
        let (cells, rows, columns) = match state.phase() {
            GamePhase::GameOver { .. } => return AiAction::Restart,
            GamePhase::Initializing { cells, rows, columns, .. } => (cells, *rows, *columns),
            GamePhase::Playing { cells, rows, columns, .. } => (cells, *rows, *columns),
            _ => return AiAction::None,
        };

        for cell in cells.iter() {
            let number = match cell.state {
                CellState::Revealed => cell.neighbor_mines,
                _ => continue,
            };

            if number == 0 {
                continue;
            }

            let neighbors = state.neighbors(cells, cell.row_id, cell.column_id, rows, columns);

            let flagged = neighbors.iter()
                .filter(|c| matches!(c.state, CellState::Flagged))
                .count();

            let hidden: Vec<_> = neighbors.iter()
                .filter(|c| matches!(c.state, CellState::Hidden))
                .collect();

            if flagged == number && !hidden.is_empty() {
                let c = hidden[0];
                return AiAction::Reveal {
                    row: c.row_id,
                    col: c.column_id,
                };
            }

            if flagged + hidden.len() == number && !hidden.is_empty() {
                let c = hidden[0];
                return AiAction::Flag {
                    row: c.row_id,
                    col: c.column_id,
                };
            }
        }

        let mut candidates: Vec<&GameCell> = cells
            .iter()
            .filter(|c| matches!(c.state, CellState::Hidden))
            .collect();

        if candidates.is_empty() {
            return AiAction::None;
        }

        let mut rng = rand::rng();
        let cell = candidates.choose(&mut rng).unwrap();

        AiAction::Reveal {
            row: cell.row_id,
            col: cell.column_id,
        }
    }

}