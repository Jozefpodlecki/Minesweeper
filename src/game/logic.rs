use rand::{rng, seq::SliceRandom};

use crate::game::model::{CellState, GameCell};

/// Creates a flat grid of cells
pub fn initialize_cells(rows: usize, cols: usize) -> Box<[GameCell]> {
    let mut cells = Vec::with_capacity(rows * cols);

    for row in 0..rows {
        for col in 0..cols {
            cells.push(GameCell::new(row, col, cols));
        }
    }

    cells.into()
}

/// Randomly places mines, avoiding a forbidden index (first click)
pub fn setup_mines(cells: &mut [GameCell], mines_count: usize, forbidden_idx: usize) {
    let mut indices: Vec<usize> = (0..cells.len())
        .filter(|&i| i != forbidden_idx)
        .collect();

    let mut rng = rng();
    indices.shuffle(&mut rng);

    for &idx in indices.iter().take(mines_count) {
        cells[idx].is_mine = true;
    }
}

/// Computes neighbor mine counts for all cells
pub fn calculate_neighbor_counts(cells: &mut [GameCell], rows: usize, cols: usize) {
    for row in 0..rows {
        for col in 0..cols {
            let idx = to_index(row, col, cols);

            if cells[idx].is_mine {
                continue;
            }

            cells[idx].neighbor_mines =
                count_adjacent_mines(cells, row, col, rows, cols);
        }
    }
}

/// Reveals a cell and recursively expands if needed
pub fn reveal_cell(
    cells: &mut [GameCell],
    revealed_count: &mut usize,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) {
    if !in_bounds(row, col, rows, cols) {
        return;
    }

    let idx = to_index(row, col, cols);

    if !can_reveal(&cells[idx]) {
        return;
    }

    reveal_single_cell(cells, revealed_count, idx);

    if should_expand(&cells[idx]) {
        reveal_neighbors(cells, revealed_count, rows, cols, row, col);
    }
}

/// -------------------- Helpers --------------------

fn reveal_neighbors(
    cells: &mut [GameCell],
    revealed_count: &mut usize,
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
) {
    for (nr, nc) in neighbors(row, col, rows, cols) {
        let nidx = to_index(nr, nc, cols);

        if matches!(cells[nidx].state, CellState::Revealed | CellState::Flagged) {
            continue;
        }

        reveal_single_cell(cells, revealed_count, nidx);

        if cells[nidx].neighbor_mines == 0 && !cells[nidx].is_mine {
            reveal_neighbors(cells, revealed_count, rows, cols, nr, nc);
        }
    }
}

fn reveal_single_cell(cells: &mut [GameCell], revealed_count: &mut usize, idx: usize) {
    cells[idx].state = CellState::Revealed;
    *revealed_count += 1;
}

fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(8);

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                result.push((nr as usize, nc as usize));
            }
        }
    }

    result
}

fn in_bounds(row: usize, col: usize, rows: usize, cols: usize) -> bool {
    row < rows && col < cols
}

fn to_index(row: usize, col: usize, cols: usize) -> usize {
    row * cols + col
}

fn can_reveal(cell: &GameCell) -> bool {
    matches!(cell.state, CellState::Hidden)
}

fn should_expand(cell: &GameCell) -> bool {
    !cell.is_mine && cell.neighbor_mines == 0
}

fn count_adjacent_mines(
    cells: &[GameCell],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> usize {
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let idx = (nr as usize) * cols + (nc as usize);
                if cells[idx].is_mine {
                    count += 1;
                }
            }
        }
    }

    count
}

mod tests {
    
}