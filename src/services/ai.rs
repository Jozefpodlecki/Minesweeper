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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{GamePhase, GameCell, CellState};

    #[test]
    fn should_restart_on_game_over() {
        let state = GameState::game_over(true);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        matches!(action, AiAction::Restart);
    }

    #[test]
    fn should_reveal_when_all_mines_flagged() {
        // Center cell = 1, one flagged neighbor, one hidden → should reveal hidden
        let cells = vec![
            GameCell::new_with_state(0, 0, 10, false, CellState::Revealed),
            GameCell::new_with_state(0, 1, 10, false, CellState::Flagged),
            GameCell::new_with_state(1, 0, 10, false, CellState::Hidden),
        ];

        let state = GameState::playing(cells, 2, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Reveal { .. } => {}
            _ => panic!("Expected Reveal action"),
        }
    }

    #[test]
    fn should_flag_when_all_hidden_are_mines() {
        // Center cell = 2, two hidden neighbors → both must be mines → flag one
        let cells = vec![
            GameCell::new_with_state(0, 0, 10, false, CellState::Revealed),
            GameCell::new_with_state(0, 1, 10, false, CellState::Hidden),
            GameCell::new_with_state(1, 0, 10, false, CellState::Hidden),
        ];

        let state = GameState::playing(cells, 2, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Flag { .. } => {}
            _ => panic!("Expected Flag action"),
        }
    }

    #[test]
    fn should_fall_back_to_random_reveal_when_no_logic() {
        let cells = vec![
            GameCell::new_with_state(0, 0, 10, false, CellState::Hidden),
            GameCell::new_with_state(0, 1, 10, false, CellState::Hidden),
        ];

        let state = GameState::playing(cells, 1, 2);
        let ai = AiAigent::new();

        let action = ai.next(&state);

        match action {
            AiAction::Reveal { .. } => {}
            _ => panic!("Expected fallback Reveal"),
        }
    }
}