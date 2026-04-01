use std::hint::unreachable_unchecked;
use std::fmt::{self, Display, Formatter};
use chrono::{DateTime, Duration, Utc};

use crate::{game::{GameCell}, services::{DefaultSystemClock, SystemClock}};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum GamePhase {
    #[default]
    Idle,
    Initializing {
        cells: Box<[GameCell]>,
        rows: usize,
        columns: usize,
        mines_count: usize,
    },
    Playing {
        cells: Box<[GameCell]>,
        rows: usize,
        columns: usize,
        mines_count: usize,
        revealed_count: usize,
        flags_count: usize,
        started_at: DateTime<Utc>,
    },
    GameOver {
        has_won: bool,
        cells: Box<[GameCell]>,
        rows: usize,
        columns: usize,
        duration: Duration,
        started_at: DateTime<Utc>,
        mines_count: usize,
        revealed_count: usize,
        flags_count: usize,
    }
}

impl Display for GamePhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GamePhase::Idle => {
                write!(f, "Idle")
            }

            GamePhase::Initializing { rows, columns, mines_count, .. } => {
                write!(
                    f,
                    "Initializing | {}x{} | mines: {}",
                    rows, columns, mines_count
                )
            }

            GamePhase::Playing {
                rows,
                columns,
                mines_count,
                revealed_count,
                flags_count,
                started_at,
                ..
            } => {
                let elapsed = chrono::Utc::now() - *started_at;

                write!(
                    f,
                    "Playing | {}x{} | mines: {} | revealed: {} | flags: {} | elapsed: {}s",
                    rows,
                    columns,
                    mines_count,
                    revealed_count,
                    flags_count,
                    elapsed.num_seconds()
                )
            }

            GamePhase::GameOver {
                has_won,
                rows,
                columns,
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
                    rows,
                    columns,
                    mines_count,
                    revealed_count,
                    flags_count,
                    duration.num_seconds()
                )
            }
        }
    }
}