// use rand::{rng, seq::SliceRandom};

// use crate::game::model::{CellState, GameCell};

// /// Creates a flat grid of cells
// pub fn initialize_cells(rows: usize, cols: usize) -> Box<[GameCell]> {
//     let mut cells = Vec::with_capacity(rows * cols);

//     for row in 0..rows {
//         for col in 0..cols {
//             cells.push(GameCell::new(row, col, cols));
//         }
//     }

//     cells.into()
// }

// pub fn get_forbidden_indices(row: usize, col: usize, rows: usize, columns: usize) -> Vec<usize> {
//     let mut forbidden = vec![row * columns + col];
    
//     for delta_row in -1..=1 {
//         for delta_col in -1..=1 {
//             if delta_row == 0 && delta_col == 0 {
//                 continue;
//             }
            
//             let neighbor_row = row as i32 + delta_row;
//             let neighbor_col = col as i32 + delta_col;
            
//             if neighbor_row >= 0 && neighbor_row < rows as i32 && neighbor_col >= 0 && neighbor_col < columns as i32 {
//                 forbidden.push(neighbor_row as usize * columns + neighbor_col as usize);
//             }
//         }
//     }
    
//     forbidden
// }

// /// Randomly places mines, avoiding a forbidden index (first click)
// pub fn setup_mines(cells: &mut [GameCell], mines_count: usize, forbidden_indices: &[usize]) {
//     let mut indices: Vec<usize> = (0..cells.len())
//         .filter(|i| !forbidden_indices.contains(i))
//         .collect();

//     let mut rng = rng();
//     indices.shuffle(&mut rng);

//     for &idx in indices.iter().take(mines_count) {
//         cells[idx].is_mine = true;
//     }
// }

// /// Computes neighbor mine counts for all cells
// pub fn calculate_neighbor_counts(cells: &mut [GameCell], rows: usize, cols: usize) {
//     for row in 0..rows {
//         for col in 0..cols {
//             let idx = to_index(row, col, cols);

//             if cells[idx].is_mine {
//                 continue;
//             }

//             cells[idx].neighbor_mines =
//                 count_adjacent_mines(cells, row, col, rows, cols);
//         }
//     }
// }

// pub fn reveal_cell(
//     cells: &mut [GameCell],
//     revealed_count: &mut usize,
//     row: usize,
//     col: usize,
//     rows: usize,
//     cols: usize,
// ) {
//     if !in_bounds(row, col, rows, cols) {
//         return;
//     }

//     let idx = to_index(row, col, cols);

//     if !can_reveal(&cells[idx]) {
//         return;
//     }

//     reveal_single_cell(cells, revealed_count, idx);

//     if should_expand(&cells[idx]) {
//         // Use a stack for iterative flood fill to avoid recursion depth issues
//         let mut stack = vec![(row, col)];
        
//         while let Some((r, c)) = stack.pop() {
//             for (nr, nc) in neighbors(r, c, rows, cols) {
//                 let nidx = to_index(nr, nc, cols);
                
//                 // Skip if already revealed or flagged
//                 if matches!(cells[nidx].state, CellState::Revealed | CellState::Flagged) {
//                     continue;
//                 }
                
//                 // Reveal the neighbor
//                 reveal_single_cell(cells, revealed_count, nidx);
                
//                 // If neighbor also has zero mines, add it to stack for further expansion
//                 if cells[nidx].neighbor_mines == 0 && !cells[nidx].is_mine {
//                     stack.push((nr, nc));
//                 }
//             }
//         }
//     }
// }

// fn reveal_single_cell(cells: &mut [GameCell], revealed_count: &mut usize, idx: usize) {
//     cells[idx].state = CellState::Revealed;
//     *revealed_count += 1;
// }

// fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
//     let mut result = Vec::with_capacity(8);

//     for dr in -1..=1 {
//         for dc in -1..=1 {
//             if dr == 0 && dc == 0 {
//                 continue;
//             }

//             let nr = row as i32 + dr;
//             let nc = col as i32 + dc;

//             if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
//                 result.push((nr as usize, nc as usize));
//             }
//         }
//     }

//     result
// }

// fn in_bounds(row: usize, col: usize, rows: usize, cols: usize) -> bool {
//     row < rows && col < cols
// }

// fn to_index(row: usize, col: usize, cols: usize) -> usize {
//     row * cols + col
// }

// fn can_reveal(cell: &GameCell) -> bool {
//     matches!(cell.state, CellState::Hidden)
// }

// fn should_expand(cell: &GameCell) -> bool {
//     !cell.is_mine && cell.neighbor_mines == 0
// }

// fn count_adjacent_mines(
//     cells: &[GameCell],
//     row: usize,
//     col: usize,
//     rows: usize,
//     cols: usize,
// ) -> usize {
//     let mut count = 0;

//     for dr in -1..=1 {
//         for dc in -1..=1 {
//             if dr == 0 && dc == 0 {
//                 continue;
//             }

//             let nr = row as i32 + dr;
//             let nc = col as i32 + dc;

//             if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
//                 let idx = (nr as usize) * cols + (nc as usize);
//                 if cells[idx].is_mine {
//                     count += 1;
//                 }
//             }
//         }
//     }

//     count
// }

// mod tests {
    
// }