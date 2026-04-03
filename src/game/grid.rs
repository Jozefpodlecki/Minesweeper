use std::{collections::HashSet, ops::{Deref, DerefMut, Index, IndexMut}, rc::Rc};

use rand::{rng, seq::SliceRandom};

use crate::game::{CellState, GameCell, SavedGameCell, SavedGameGrid};

#[derive(Clone, Debug, PartialEq)]
pub struct GameCells(Rc<[GameCell]>);

impl GameCells {
    pub fn new(cells: Vec<GameCell>) -> Self {
        Self(Rc::from(cells))
    }

    pub fn as_slice(&self) -> &[GameCell] {
        &self.0
    }

    pub fn get(&self, idx: usize) -> Option<&GameCell> {
        self.0.get(idx)
    }

    pub fn get_mut(&mut self) -> &mut [GameCell] {
        Rc::make_mut(&mut self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for GameCells {
    type Target = [GameCell];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for GameCells {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::get_mut(&mut self.0).unwrap()
    }
}

impl Index<usize> for GameCells {
    type Output = GameCell;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for GameCells {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        Rc::make_mut(&mut self.0).get_mut(idx).expect("index out of bounds")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameGrid {
    cells: Box<[GameCell]>,
    pub rows: usize,
    pub columns: usize,
}

impl GameGrid {
    pub fn new(rows: usize, columns: usize) -> Self {
        let cells = (0..rows * columns)
            .map(|i| GameCell::new(i / columns, i % columns, columns))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { cells, rows, columns }
    }

    pub fn from_saved(value: SavedGameGrid) -> Self {
        let mut grid = Self {
            cells: value.cells.into_iter().map(GameCell::from_saved).collect(),
            rows: value.rows,
            columns: value.columns,
        };

        grid.calculate_neighbor_counts();

        grid
    }

    pub fn cells(&self) -> &[GameCell] {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut [GameCell] {
        &mut self.cells
    }

    pub fn index(&self, row: usize, col: usize) -> usize {
        row * self.columns + col
    }

    pub fn at(&self, row: usize, col: usize) -> &GameCell {
        &self.cells[row * self.columns + col]
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut GameCell {
        &mut self.cells[row * self.columns + col]
    }

    pub fn iter(&self) -> impl Iterator<Item = &GameCell> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut GameCell> {
        self.cells.iter_mut()
    }
    
    pub fn len(&self) -> usize { self.cells.len() }

    pub fn is_mine(&self, idx: usize) -> bool {
        self.cells[idx].is_mine
    }
    
    pub fn is_revealed(&self, idx: usize) -> bool {
        matches!(self.cells[idx].state, CellState::Revealed)
    }

    pub fn is_flagged(&self, idx: usize) -> bool {
        matches!(self.cells[idx].state, CellState::Flagged)
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) -> CellState {
        let idx = self.index(row, col);
        let cell = &mut self.cells[idx];
        match cell.state {
            CellState::Hidden => { cell.state = CellState::Flagged; CellState::Flagged }
            CellState::Flagged => { cell.state = CellState::Hidden; CellState::Hidden }
            CellState::Revealed => CellState::Revealed,
        }
    }

    pub fn reveal_cell(&mut self, row: usize, col: usize) -> usize {
        if !matches!(self.at(row, col).state, CellState::Hidden) {
            return 0;
        }
        
        let mut stack = vec![(row, col)];
        let mut revealed = 0;

        while let Some((r, c)) = stack.pop() {
            let cell = self.at_mut(r, c);
            if matches!(cell.state, CellState::Hidden) {
                cell.state = CellState::Revealed;
                revealed += 1;
                if cell.neighbor_mines == 0 && !cell.is_mine {
                    for (nr, nc) in self.neighbor_coords(r, c) {
                        stack.push((nr, nc));
                    }
                }
            }
        }
        revealed
    }

    fn reveal_recursive(&mut self, row: usize, col: usize, revealed_count: &mut usize) {
        if !self.in_bounds(row, col) { return; }
        let idx = self.index(row, col);
        if !matches!(self.cells[idx].state, CellState::Hidden) { return; }

        self.cells[idx].state = CellState::Revealed;
        *revealed_count += 1;

        if self.cells[idx].neighbor_mines == 0 && !self.cells[idx].is_mine {
            let coords: Vec<_> = self.neighbor_coords(row, col).collect();

            for (nr, nc) in coords {
                self.reveal_recursive(nr, nc, revealed_count);
            }
        }
    }

    pub fn reveal_all(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.state = CellState::Revealed;
        }
    }

    /// Indices of the selected cell and its neighbors (max 9 total).
    ///
    /// Used to exclude these positions from mine placement so the first
    /// revealed cell is always safe.
    pub fn forbidden_indices(&self, row: usize, col: usize) -> impl Iterator<Item = usize> {
        std::iter::once(self.index(row, col))
            .chain(
                self.neighbor_coords(row, col)
                    .map(move |(r, c)| self.index(r, c))
            )
    }

    pub fn setup_mines<I>(&mut self, mines_count: usize, forbidden_indices: I)
    where
        I: IntoIterator<Item = usize>,
    {
        let forbidden: HashSet<_> = forbidden_indices.into_iter().collect();

        let mut indices: Vec<usize> = (0..self.cells.len())
            .filter(|i| !forbidden.contains(i))
            .collect();

        let mut rng = rng();
        indices.shuffle(&mut rng);

        self.set_mines(&indices[..mines_count]);
    }

    pub fn set_mines(&mut self, indices: &[usize]) {
        for &idx in indices {
            self.cells[idx].is_mine = true;
        }
    }

    pub fn calculate_neighbor_counts(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.columns {
                let idx = self.index(row, col);
                if self.cells[idx].is_mine { continue; }
                self.cells[idx].neighbor_mines = self.count_adjacent_mines(row, col);
            }
        }
    }

    pub fn count_adjacent_mines(&self, row: usize, col: usize) -> usize {
        self.neighbor_coords(row, col)
            .filter(|(nr, nc)| self.cells[self.index(*nr, *nc)].is_mine)
            .count()
    }

    // pub fn neighbor_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    //     let mut result = Vec::with_capacity(8);
    //     for dr in [-1isize, 0, 1] {
    //         for dc in [-1isize, 0, 1] {
    //             if dr == 0 && dc == 0 { continue; }
    //             let nr = row as isize + dr;
    //             let nc = col as isize + dc;
    //             if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.columns as isize {
    //                 result.push((nr as usize, nc as usize));
    //             }
    //         }
    //     }
    //     result
    // }

    pub fn neighbor_coords(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.rows as isize;
        let cols = self.columns as isize;

        [-1isize, 0, 1]
            .into_iter()
            .flat_map(move |dr| {
                [-1isize, 0, 1].into_iter().map(move |dc| (dr, dc))
            })
            .filter(move |&(dr, dc)| !(dr == 0 && dc == 0))
            .filter_map(move |(dr, dc)| {
                let nr = row as isize + dr;
                let nc = col as isize + dc;

                if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                    Some((nr as usize, nc as usize))
                } else {
                    None
                }
            })
    }

    pub fn neighbors(&self, cell: &GameCell) -> impl Iterator<Item = &GameCell> {
        let row = cell.row_id;
        let col = cell.column_id;
        let rows = self.rows as isize;
        let cols = self.columns as isize;

        [-1isize, 0, 1]
            .into_iter()
            .flat_map(move |dr| {
                [-1isize, 0, 1].into_iter().map(move |dc| (dr, dc))
            })
            .filter(move |&(dr, dc)| !(dr == 0 && dc == 0))
            .filter_map(move |(dr, dc)| {
                let nr = row as isize + dr;
                let nc = col as isize + dc;

                if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                    let idx = nr as usize * self.columns + nc as usize;
                    Some(&self.cells[idx])
                } else {
                    None
                }
            })
    }

    //  pub fn neighbor_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    //     let mut result = Vec::with_capacity(8);
    //     for dr in -1..=1 {
    //         for dc in -1..=1 {
    //             if dr == 0 && dc == 0 { continue; }
    //             let nr = row as i32 + dr;
    //             let nc = col as i32 + dc;
    //             if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.columns as i32 {
    //                 result.push((nr as usize, nc as usize));
    //             }
    //         }
    //     }
    //     result
    // }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.columns
    }

    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.reset();
        }
    }
}