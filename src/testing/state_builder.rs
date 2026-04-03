use chrono::Utc;

use crate::{
    game::{CellState, GameCell, GameGrid, GamePhase, GameSettings, GameState},
    services::{DefaultSystemClock, SystemClock},
};

#[allow(unused)]
pub struct GameStateBuilder {
    settings: GameSettings,
    rows: usize,
    columns: usize,
    mines_count: usize,
    phase: GamePhase,
}

#[allow(unused)]
impl GameStateBuilder {
    pub fn new(rows: usize, columns: usize, mines_count: usize) -> Self {
        let settings = GameSettings { rows, columns, mines_count };

        Self {
            settings,
            rows,
            columns,
            mines_count,
            phase: Default::default(),
        }
    }

    fn grid(phase: &GamePhase) -> &GameGrid {
        match phase {
            GamePhase::Idle => panic!("Cannot modify grid in Idle phase"),
            GamePhase::Initializing { grid, .. } => grid,
            GamePhase::Playing { grid, .. } => grid,
            GamePhase::GameOver { grid, .. } => grid,
        }
    }

    fn grid_mut(phase: &mut GamePhase) -> &mut GameGrid {
        match phase {
            GamePhase::Idle => panic!("Cannot modify grid in Idle phase"),
            GamePhase::Initializing { grid, .. } => grid,
            GamePhase::Playing { grid, .. } => grid,
            GamePhase::GameOver { grid, .. } => grid,
        }
    }

    pub fn mine(mut self, row: usize, col: usize) -> Self {
        let idx = row * self.columns + col;
        Self::grid_mut(&mut self.phase).cells_mut()[idx].is_mine = true;
        self.mines_count += 1;
        self
    }

    pub fn revealed(mut self, row: usize, col: usize) -> Self {
        Self::grid_mut(&mut self.phase).at_mut(row, col).state = CellState::Revealed;
        self
    }

    pub fn flagged(mut self, row: usize, col: usize) -> Self {
        Self::grid_mut(&mut self.phase).at_mut(row, col).state = CellState::Flagged;
        self
    }

    pub fn neighbor_count(mut self, row: usize, col: usize, count: usize) -> Self {
        Self::grid_mut(&mut self.phase).at_mut(row, col).neighbor_mines = count;
        self
    }

    pub fn count_revealed(grid: &GameGrid) -> usize {
        grid
            .cells()
            .iter()
            .filter(|c| matches!(c.state, CellState::Revealed))
            .count()
    }

    pub fn count_flags(grid: &GameGrid) -> usize {
        grid
            .cells()
            .iter()
            .filter(|c| matches!(c.state, CellState::Flagged))
            .count()
    }

    pub fn playing_with_mines(mut self, row: usize, col: usize, mines: &[usize]) -> Self {
        let mut grid = GameGrid::new(self.rows, self.columns);
        grid.set_mines(mines);

        let revealed_count = Self::count_revealed(&grid);
        let flags_count = Self::count_flags(&grid);

        self.phase = GamePhase::Playing {
            grid,
            mines_count: self.mines_count,
            revealed_count,
            flags_count,
            started_at: Utc::now(),
        };
        self
    }

    pub fn playing_with_random_mines(mut self, row: usize, col: usize) -> Self {
        let mut grid = GameGrid::new(self.rows, self.columns);
        let forbidden_indices: Vec<_> = grid.forbidden_indices(row, col).collect();
        grid.setup_mines(self.mines_count, forbidden_indices);
        grid.calculate_neighbor_counts();

        let revealed_count = Self::count_revealed(&grid);
        let flags_count = Self::count_flags(&grid);

        self.phase = GamePhase::Playing {
            grid,
            mines_count: self.mines_count,
            revealed_count,
            flags_count,
            started_at: Utc::now(),
        };
        self
    }

    pub fn initializing(mut self) -> Self {
        let grid = GameGrid::new(self.rows, self.columns);
        self.phase = GamePhase::Initializing {
            grid,
            mines_count: self.mines_count,
        };
        self
    }

    pub fn print(&self) {
        use crate::game::CellState::*;

        let grid = match &self.phase {
            GamePhase::Idle => {
                println!("Grid is in Idle phase (empty).");
                return;
            }
            GamePhase::Initializing { grid, .. } => grid,
            GamePhase::Playing { grid, .. } => grid,
            GamePhase::GameOver { grid, .. } => grid,
        };

        let mut output = String::new();
        output.push_str("   ");
        for c in 0..self.columns {
            output.push_str(&format!(" {} ", c));
        }
        output.push('\n');

        output.push_str("  +");
        for _ in 0..self.columns {
            output.push_str("---");
        }
        output.push_str("+\n");

        for r in 0..self.rows {
            output.push_str(&format!("{r} |"));
            for c in 0..self.columns {
                let cell = grid.at(r, c);
                let symbol = match cell.state {
                    Hidden => '■',
                    Flagged => '⚑',
                    Revealed => {
                        if cell.is_mine { '*' }
                        else if cell.neighbor_mines > 0 {
                            std::char::from_digit(cell.neighbor_mines as u32, 10).unwrap()
                        } else { ' ' }
                    }
                };
                output.push_str(&format!(" {} ", symbol));
            }
            output.push_str("|\n");
        }

        output.push_str("  +");
        for _ in 0..self.columns {
            output.push_str("---");
        }
        output.push_str("+\n");

        output.push_str(&format!("Mines count: {}\n", self.mines_count));
        output.push_str(&format!("Revealed count: {}\n", GameStateBuilder::count_revealed(grid)));
        output.push_str(&format!("Flags count: {}\n", GameStateBuilder::count_flags(grid)));

        println!("{output}");
    }

    pub fn game_over(mut self, has_won: bool) -> Self {

        match self.phase {
            GamePhase::Playing { .. } => {},
            _ => panic!("Invalid phase")
        }

        let grid = GameGrid::new(self.rows, self.columns);
        let revealed_count = Self::count_revealed(&grid);
        let flags_count = Self::count_flags(&grid);
        let last_cell = if has_won {
            None
        } else {
            Self::grid_mut(&mut self.phase)
                .cells()
                .iter()
                .find(|c| c.is_mine)
                .cloned()
        };

        self.phase = GamePhase::GameOver {
            has_won,
            grid,
            last_cell,
            duration: Default::default(),
            started_at: Utc::now(),
            mines_count: self.mines_count,
            revealed_count,
            flags_count,
        };

        self
    }

    pub fn build(self) -> GameState<DefaultSystemClock> {
        self.build_with_clock(DefaultSystemClock)
    }

    pub fn build_with_clock<SC: SystemClock + Clone>(self, clock: SC) -> GameState<SC> {
        GameState::test(clock, self.settings, self.phase)
    }
}