use std::fmt::{self, Display, Formatter};
use log::*;
use rand::{seq::{IndexedRandom, IteratorRandom}, RngExt};

use crate::{game::{CellState, GameCell, GamePhase, GameState}, services::SystemClock};

/// Describes *why* the AI selected a particular action.
///
/// This is used for debugging, analysis, and potentially UI feedback
/// (e.g. distinguishing between guaranteed moves and guesses).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiReason {
    
    /// A logically guaranteed move based on the current board state.
    ///
    /// The AI applied deterministic rules (e.g. all mines flagged or all
    /// remaining neighbors must be mines), meaning the action is
    /// provably correct and carries zero risk.
    Deterministic,
    
    /// An informed guess based on partial information.
    ///
    /// The AI evaluates local probabilities (e.g. remaining mines vs
    /// hidden neighbors) and selects the lowest-risk option available.
    /// This is not guaranteed to be safe, but minimizes expected risk.
    Probabilistic,
    
    /// A completely uninformed guess.
    ///
    /// No useful deterministic or probabilistic inference could be made,
    /// so the AI selects a random hidden cell. This carries the highest
    /// risk and is used only as a last resort.
    Random,
}

impl Display for AiReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AiReason::Deterministic => write!(f, "Deterministic"),
            AiReason::Probabilistic => write!(f, "Probabilistic"),
            AiReason::Random => write!(f, "Random"),
        }
    }
}

pub enum AiAction {
    None,
    Reveal {
        column: usize,
        row: usize,
        reason: AiReason,
    },
    Flag {
        column: usize,
        row: usize,
        reason: AiReason
    },
    Restart
}

impl Display for AiAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AiAction::None => write!(f, "AiAction::None"),
            AiAction::Reveal { reason, column, row } => write!(f, "AiAction::Reveal Reason={}, Row={}, Column={}", reason, column, row),
            AiAction::Flag { reason, column, row } => write!(f, "AiAction::Flag Reason={}, Row={} Column={}", reason, column, row),
            AiAction::Restart => write!(f, "AiAction::Restart"),
        }
    }
}

pub struct AiAigent;

impl AiAigent {
    pub fn new() -> Self {
        Self{}
    }

    pub fn next<SC: SystemClock + Clone>(&self, state: &GameState<SC>) -> AiAction {
        debug!("{}", state.phase());

        match state.phase() {
            GamePhase::GameOver { .. } => return AiAction::Restart,

            GamePhase::Initializing { rows, columns, .. } => {
                return self.random_reveal(*rows, *columns);
            }

            GamePhase::Playing { cells, rows, columns, .. } => {
                if let Some(action) = self.try_deterministic(cells, *rows, *columns, state) {
                    return action;
                }

                if let Some(action) = self.try_probabilistic(cells, *rows, *columns, state) {
                    return action;
                }

                return self.random_from_hidden(cells);
            }

            _ => AiAction::None,
        }
    }

    fn try_deterministic<SC: SystemClock + Clone>(
        &self,
        cells: &[GameCell],
        rows: usize,
        columns: usize,
        state: &GameState<SC>,
    ) -> Option<AiAction> {
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

            if hidden.is_empty() {
                continue;
            }

 
            if flagged == number {
                let c = hidden[0];
                return Some(AiAction::Reveal {
                    row: c.row_id,
                    column: c.column_id,
                    reason: AiReason::Deterministic
                });
            }

            if flagged + hidden.len() == number {
                let c = hidden[0];
                return Some(AiAction::Flag {
                    row: c.row_id,
                    column: c.column_id,
                    reason: AiReason::Deterministic
                });
            }
        }

        None
    }

    fn try_probabilistic<SC: SystemClock + Clone>(
        &self,
        cells: &[GameCell],
        rows: usize,
        columns: usize,
        state: &GameState<SC>,
    ) -> Option<AiAction> {
        let mut best: Option<(&GameCell, f64)> = None;

        for cell in cells.iter().filter(|c| matches!(c.state, CellState::Revealed)) {
            let number = cell.neighbor_mines;
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

            if hidden.is_empty() {
                continue;
            }

            let remaining_mines = (number as isize - flagged as isize).max(0) as f64;
            let risk = remaining_mines / hidden.len() as f64;

            for &h in &hidden {
                match best {
                    None => best = Some((h, risk)),
                    Some((_, best_risk)) if risk < best_risk => {
                        best = Some((h, risk));
                    }
                    _ => {}
                }
            }
        }

        best.map(|(cell, _)| AiAction::Reveal {
            row: cell.row_id,
            column: cell.column_id,
            reason: AiReason::Probabilistic
        })
    }

    fn random_from_hidden(&self, cells: &[GameCell]) -> AiAction {
        let mut rng = rand::rng();

        let candidate = cells
            .iter()
            .filter(|c| matches!(c.state, CellState::Hidden))
            .choose(&mut rng);

        if let Some(cell) = candidate {
            AiAction::Reveal {
                row: cell.row_id,
                column: cell.column_id,
                reason: AiReason::Random,
            }
        } else {
            AiAction::None
        }
    }

    fn random_reveal(&self, rows: usize, columns: usize) -> AiAction {
        let mut rng = rand::rng();

        AiAction::Reveal {
            row: rng.random_range(0..rows),
            column: rng.random_range(0..columns),
            reason: AiReason::Random,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{GamePhase, GameCell, CellState};

    #[test]
    fn should_restart_on_game_over() {
        let state = GameState::game_over(true);
        let ai = AiAigent::new();
        let action = ai.next(&state);
        assert!(matches!(action, AiAction::Restart));
    }

    #[test]
    fn should_reveal_when_all_mines_flagged() {
        let cells = vec![
            GameCell::test(0, 0, 2, false, 1, CellState::Revealed),
            GameCell::test(0, 1, 2, false, 1, CellState::Flagged),
            GameCell::test(1, 0, 2, false, 1, CellState::Hidden),
            GameCell::test(1, 1, 2, false, 1, CellState::Hidden),
        ];

        let state = GameState::playing(cells.clone(), 2, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Reveal { row, column, reason } => {
                assert_eq!(reason, AiReason::Deterministic);
                assert_eq!((row, column), (1, 0));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_flag_when_all_hidden_are_mines() {
        let cells = vec![
            GameCell::test(0, 0, 2, false, 2, CellState::Revealed),
            GameCell::test(0, 1, 2, false, 0, CellState::Hidden),
            GameCell::test(1, 0, 2, false, 0, CellState::Hidden),
        ];

        let state = GameState::playing(cells.clone(), 2, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Flag { row, column, reason } => {
                assert_eq!(reason, AiReason::Deterministic);
                assert!((row, column) == (0, 1) || (row, column) == (1, 0));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_fall_back_to_random_reveal_when_no_logic() {
        let cells = vec![
            GameCell::test(0, 0, 2, false, 0, CellState::Hidden),
            GameCell::test(0, 1, 2, false, 0, CellState::Hidden),
        ];

        let state = GameState::playing(cells.clone(), 1, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Reveal { row, column, reason } => {
                assert_eq!(reason, AiReason::Random);
                assert!(row < 1);
                assert!(column < 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_prefer_probabilistic_over_random_when_available() {
        let cells = vec![
            GameCell::test(0, 0, 2, false, 1, CellState::Revealed),
            GameCell::test(0, 1, 2, false, 0, CellState::Hidden),
            GameCell::test(1, 0, 2, false, 0, CellState::Hidden),
        ];

        let state = GameState::playing(cells.clone(), 2, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Reveal { reason, row, column } => {
                assert_eq!(reason, AiReason::Probabilistic);
                assert!((row, column) == (0, 1) || (row, column) == (1, 0));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_return_none_when_no_hidden_cells_exist() {
        let cells = vec![
            GameCell::test(0, 0, 1, false, 0, CellState::Revealed),
            GameCell::test(0, 1, 1, false, 0, CellState::Revealed),
        ];

        let state = GameState::playing(cells.clone(), 1, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        assert!(matches!(action, AiAction::None));
    }

    #[test]
    fn should_handle_initializing_phase_within_bounds() {
        let state = GameState::initializing(5, 5);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Reveal { row, column, reason } => {
                assert_eq!(reason, AiReason::Random);
                assert!(row < 5);
                assert!(column < 5);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn should_not_flag_or_reveal_when_no_relevant_cells() {
        let cells = vec![
            GameCell::test(0, 0, 1, false, 0, CellState::Revealed),
        ];

        let state = GameState::playing(cells.clone(), 1, 1);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        assert!(matches!(action, AiAction::None));
    }

    #[test]
    fn probabilistic_selects_lowest_risk_cell_among_multiple() {
        let cells = vec![
            GameCell::test(0, 0, 2, false, 2, CellState::Revealed),
            GameCell::test(0, 1, 2, false, 0, CellState::Hidden),
            GameCell::test(1, 0, 2, false, 0, CellState::Hidden),
            GameCell::test(1, 1, 2, false, 0, CellState::Hidden),
        ];

        let state = GameState::playing(cells.clone(), 2, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Reveal { reason, row, column } => {
                assert_eq!(reason, AiReason::Probabilistic);
                assert!((row, column) == (0, 1)
                    || (row, column) == (1, 0)
                    || (row, column) == (1, 1));
            }
            _ => panic!(),
        }
    }
}