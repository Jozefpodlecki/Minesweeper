use std::fmt::{self, Display, Formatter};
use chrono::{DateTime, Duration, Utc};
use crate::game::{GameCell, GameGrid};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum GamePhase {
    #[default]
    Idle,
    Initializing {
        grid: GameGrid,
        mines_count: usize,
    },
    Playing {
        grid: GameGrid,
        mines_count: usize,
        revealed_count: usize,
        flags_count: usize,
        started_at: DateTime<Utc>,
    },
    GameOver {
        has_won: bool,
        grid: GameGrid,
        last_cell: Option<GameCell>,
        duration: Duration,
        started_at: DateTime<Utc>,
        mines_count: usize,
        revealed_count: usize,
        flags_count: usize,
    },
}

impl Display for GamePhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GamePhase::Idle => write!(f, "Idle"),

            GamePhase::Initializing { grid, mines_count } => {
                write!(
                    f,
                    "Initializing | {}x{} | mines: {}",
                    grid.rows, grid.columns, mines_count
                )
            }

            GamePhase::Playing {
                grid,
                mines_count,
                revealed_count,
                flags_count,
                started_at,
            } => {
                let elapsed = Utc::now() - *started_at;
                write!(
                    f,
                    "Playing | {}x{} | mines: {} | revealed: {} | flags: {} | elapsed: {}s",
                    grid.rows,
                    grid.columns,
                    mines_count,
                    revealed_count,
                    flags_count,
                    elapsed.num_seconds()
                )
            }

            GamePhase::GameOver {
                has_won,
                grid,
                duration,
                mines_count,
                revealed_count,
                flags_count,
                ..
            } => {
                write!(
                    f,
                    "GameOver | {} | {}x{} | mines: {} | revealed: {} | flags: {} | duration: {}s",
                    if *has_won { "WIN" } else { "LOSS" },
                    grid.rows,
                    grid.columns,
                    mines_count,
                    revealed_count,
                    flags_count,
                    duration.num_seconds()
                )
            }
        }
    }
}