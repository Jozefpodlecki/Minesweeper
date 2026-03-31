use chrono::Utc;

use crate::{game::Repository, models::GameResult};


pub fn set_sample_records(repository: &Repository) {
    let now = Utc::now();

    repository.set_last_record(GameResult {
        has_won: true,
        started_at: now - chrono::Duration::minutes(25),
        duration: chrono::Duration::minutes(5),
        rows: 10,
        columns: 10,
        revealed_count: 20,
        mines_count: 50,
        flags_count: 20
    });

    repository.set_last_record(GameResult {
        has_won: false,
        started_at: now - chrono::Duration::minutes(15),
        duration: chrono::Duration::minutes(5),
        rows: 10,
        columns: 10,
        revealed_count: 20,
        mines_count: 50,
        flags_count: 20
    });

    repository.set_last_record(GameResult {
        has_won: true,
        started_at: now,
        duration: chrono::Duration::minutes(5),
        rows: 10,
        columns: 10,
        revealed_count: 20,
        mines_count: 50,
        flags_count: 20
    });
}