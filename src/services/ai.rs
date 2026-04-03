use std::fmt::{self, Display, Formatter};
use log::*;
use rand::{seq::{IndexedRandom, IteratorRandom}, RngExt};

use crate::{game::{CellState, GameCell, GameGrid, GamePhase, GameState}, services::SystemClock};

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

            GamePhase::Initializing { grid, .. } => {
                return self.random_reveal(grid.rows, grid.columns);
            }

            GamePhase::Playing { grid, .. } => {
                if let Some(action) = self.try_deterministic(grid, state) {
                    return action;
                }

                if let Some(action) = self.try_probabilistic(grid, state) {
                    return action;
                }

                return self.random_from_hidden(grid.cells());
            }

            _ => AiAction::None,
        }
    }

    fn try_deterministic<SC: SystemClock + Clone>(
        &self,
        grid: &GameGrid,
        state: &GameState<SC>,
    ) -> Option<AiAction> {
        for cell in grid.cells().iter() {
            let number = match cell.state {
                CellState::Revealed => cell.neighbor_mines,
                _ => continue,
            };

            if number == 0 {
                continue;
            }

            let (flagged, hidden) = grid
                .neighbors(cell)
                .fold((0, Vec::new()), |(mut flagged, mut hidden), c| {
                    match c.state {
                        CellState::Flagged => flagged += 1,
                        CellState::Hidden => hidden.push(c),
                        _ => {}
                    }
                    (flagged, hidden)
                });

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

    fn try_probabilistic<SC: SystemClock + Clone>(&self, grid: &GameGrid, state: &GameState<SC>) -> Option<AiAction> {
        let mut best: Option<(&GameCell, f64)> = None;

        for cell in grid.cells().iter().filter(|c| matches!(c.state, CellState::Revealed)) {
            let number = cell.neighbor_mines;
            if number == 0 {
                continue;
            }

            let (flagged, hidden) = grid
                .neighbors(cell)
                .fold((0, Vec::new()), |(mut flagged, mut hidden), c| {
                    match c.state {
                        CellState::Flagged => flagged += 1,
                        CellState::Hidden => hidden.push(c),
                        _ => {}
                    }
                    (flagged, hidden)
                });

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
    use crate::{
        game::{CellState, GamePhase},
        testing::GameStateBuilder,
    };

    #[test]
    fn should_restart_on_game_over() {
        let state = GameStateBuilder::new(5, 5, 5)
            .playing_with_random_mines(0, 0)
            .mine(0, 0)
            .mine(1, 1)
            .game_over(false)
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        assert!(matches!(action, AiAction::Restart));
    }

    #[test]
    fn should_reveal_when_all_mines_flagged() {
        let state = GameStateBuilder::new(2, 2, 0)
            .playing_with_random_mines(0, 0)
            .revealed(0, 0)
            .flagged(0, 1)
            .neighbor_count(0, 0, 1)
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        match action {
            AiAction::Reveal { row, column, reason } => {
                assert_eq!(reason, AiReason::Deterministic);
                assert_eq!((row, column), (1, 0));
            }
            _ => panic!("Expected deterministic reveal"),
        }
    }

    #[test]
    fn should_flag_when_all_hidden_are_mines() {
        let state = GameStateBuilder::new(1, 2, 0)
            .playing_with_mines(0, 0, &[1])
            .revealed(0, 0)
            .mine(0, 1)
            .neighbor_count(0, 0, 1)
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        match action {
            AiAction::Flag { row, column, reason } => {
                assert_eq!(reason, AiReason::Deterministic);
                assert!((row, column) == (0, 1) || (row, column) == (1, 0));
            }
            flag => panic!("Expected deterministic flag. got {flag}"),
        }
    }

    #[test]
    fn should_fall_back_to_random_reveal_when_no_logic() {
        let state = GameStateBuilder::new(1, 2, 0)
            .playing_with_random_mines(0, 0)
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        match action {
            AiAction::Reveal { row, column, reason } => {
                assert_eq!(reason, AiReason::Random);
                assert!(row < 1 && column < 2);
            }
            _ => panic!("Expected random reveal"),
        }
    }

    #[test]
    fn should_prefer_probabilistic_over_random_when_available() {
        let state = GameStateBuilder::new(2, 2, 0)
            .playing_with_random_mines(0, 0)
            .revealed(0, 0)
            .neighbor_count(0, 0, 1)
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        match action {
            AiAction::Reveal { reason, row, column } => {
                assert_eq!(reason, AiReason::Probabilistic);
                assert!((row, column) == (0, 1) || (row, column) == (1, 0));
            }
            flag => panic!("Expected probabilistic reveal. got {flag}"),
        }
    }

    #[test]
    fn should_return_none_when_no_hidden_cells_exist() {
        let state = GameStateBuilder::new(1, 2, 0)
            .playing_with_random_mines(0, 0)
            .revealed(0, 0)
            .revealed(0, 1)
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        assert!(matches!(action, AiAction::None));
    }

    #[test]
    fn should_handle_initializing_phase_within_bounds() {
        let state = GameStateBuilder::new(5, 5, 5)
            .initializing()
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        match action {
            AiAction::Reveal { row, column, reason } => {
                assert_eq!(reason, AiReason::Random);
                assert!(row < 5 && column < 5);
            }
            _ => panic!("Expected random reveal"),
        }
    }

    #[test]
    fn should_not_flag_or_reveal_when_no_relevant_cells() {
        let state = GameStateBuilder::new(1, 1, 0)
            .playing_with_random_mines(0, 0)
            .revealed(0, 0)
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        assert!(matches!(action, AiAction::None));
    }

    #[test]
    fn probabilistic_selects_lowest_risk_cell_among_multiple() {
        let state = GameStateBuilder::new(2, 2, 0)
            .playing_with_random_mines(0, 0)
            .revealed(0, 0)
            .neighbor_count(0, 0, 2)
            .build();

        let ai = AiAigent::new();
        let action = ai.next(&state);

        match action {
            AiAction::Reveal { reason, row, column } => {
                assert_eq!(reason, AiReason::Probabilistic);
                assert!((row, column) == (0, 1)
                    || (row, column) == (1, 0)
                    || (row, column) == (1, 1));
            }
            _ => panic!("Expected probabilistic reveal"),
        }
    }
}