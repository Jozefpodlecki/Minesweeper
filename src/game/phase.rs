use std::hint::unreachable_unchecked;

use chrono::{DateTime, Duration, Utc};
use gloo::console::info;
use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::{game::{GameCell}, services::{DefaultSystemClock, SystemClock}};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum GamePhase {
    #[default]
    Idle,
    Initializing {
        cells: Box<[GameCell]>,
        rows: usize,
        cols: usize,
        mines_count: usize,
    },
    Playing {
        cells: Box<[GameCell]>,
        rows: usize,
        cols: usize,
        mines_count: usize,
        revealed_count: usize, 
        started_at: DateTime<Utc>,
    },
    GameOver {
        has_won: bool,
        cells: Box<[GameCell]>,
        rows: usize,
        cols: usize,
        duration: Duration,
        started_at: DateTime<Utc>,
        mines_count: usize,
        revealed_count: usize, 
    }
}
