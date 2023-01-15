use super::grid::Grid;
use itertools::Itertools;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum FieldCell {
    Empty = 0,
    Filled = 1,
    Marked = 2,
}

impl Default for FieldCell {
    fn default() -> Self {
        FieldCell::Empty
    }
}

impl From<FieldCell> for u8 {
    fn from(cell: FieldCell) -> Self {
        cell as u8
    }
}

impl From<u8> for FieldCell {
    fn from(cell: u8) -> Self {
        match cell {
            1 => FieldCell::Filled,
            2 => FieldCell::Marked,
            _ => FieldCell::Empty,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HintCell {
    pub number: u8,    // 0 represents empty field
    pub crossed: bool, // player can mark hints
}

pub struct Board {
    width: usize,
    field: Grid<FieldCell>,
    solution: Grid<FieldCell>,
    col_hints: Grid<HintCell>,
    row_hints: Grid<HintCell>,
}

impl Board {
    fn generate_col_hints(&mut self) {
        for col in 0..self.width {
            let counts: Vec<u8> = (0..self.width)
                .map(|idx| self.solution[idx][col])
                .map(|cell| (cell, 1u8))
                .coalesce(|(cell1, count1), (cell2, count2)| {
                    if cell1 == cell2 {
                        Ok((cell1, count1 + count2))
                    } else {
                        Err(((cell1, count1), (cell2, count2)))
                    }
                })
                .filter(|(cell, _)| *cell == FieldCell::Filled)
                .map(|(_, count)| count)
                .collect();
            for zip in (0..self.hint_len()).rev().zip_longest(counts.iter().rev()) {
                match zip {
                    itertools::EitherOrBoth::Left(idx) => {
                        self.col_hints[idx][col].number = 0;
                    }
                    itertools::EitherOrBoth::Both(idx, val) => {
                        self.col_hints[idx][col].number = *val;
                    }
                    itertools::EitherOrBoth::Right(_) => {}
                }
            }
        }
    }

    fn generate_row_hints(&mut self) {
        for row in 0..self.width {
            let counts: Vec<u8> = self.solution[row]
                .iter()
                .map(|cell| (*cell, 1u8))
                .coalesce(|(cell1, count1), (cell2, count2)| {
                    if cell1 == cell2 {
                        Ok((cell1, count1 + count2))
                    } else {
                        Err(((cell1, count1), (cell2, count2)))
                    }
                })
                .filter(|(cell, _)| *cell == FieldCell::Filled)
                .map(|(_, count)| count)
                .collect();
            for zip in self.row_hints[row]
                .iter_mut()
                .rev()
                .zip_longest(counts.iter().rev())
            {
                match zip {
                    itertools::EitherOrBoth::Left(cell) => {
                        cell.number = 0;
                    }
                    itertools::EitherOrBoth::Both(cell, val) => {
                        cell.number = *val;
                    }
                    itertools::EitherOrBoth::Right(_) => {}
                }
            }
        }
    }

    fn generate_hints(&mut self) {
        self.generate_col_hints();
        self.generate_row_hints();
    }
}

impl Board {
    pub fn new() -> Board {
        let width = 10;
        let height = width;
        let fields = (0..width * width).map(|i| {
            if i % 3 == 0 || i % 7 == 0 || i % 11 == 0 {
                FieldCell::Filled
            } else {
                FieldCell::Empty
            }
        });
        let hint_len = (width + 1) / 2;
        let mut result = Board {
            width,
            field: Grid::new(width, width),
            solution: Grid::from_flat(width, &Vec::<FieldCell>::from_iter(fields)),
            col_hints: Grid::new(width, hint_len),
            row_hints: Grid::new(hint_len, height),
        };
        result.generate_hints();
        result
    }

    pub fn resize(&mut self, new_width: usize) {
        self.width = new_width;
        self.field = self.field.resized(new_width, new_width);
        self.solution = self.solution.resized(new_width, new_width);
        let hint_len = (new_width + 1) / 2;
        self.col_hints = Grid::new(new_width, hint_len);
        self.row_hints = Grid::new(hint_len, new_width);
    }

    pub fn from_serialized_solution(serialized_solution: &str) -> Self {
        let solution = Grid::<FieldCell>::from_base64(serialized_solution)
            .unwrap_or_else(|_| Grid::new(10, 10));
        let (width, height) = (solution.width(), solution.height());
        let col_hint_len = (width + 1) / 2;
        let row_hint_len = (height + 1) / 2;
        let mut result = Board {
            width: solution.width(),
            field: Grid::new(width, height),
            solution,
            col_hints: Grid::new(width, col_hint_len),
            row_hints: Grid::new(row_hint_len, height),
        };
        result.generate_hints();
        result
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn hint_len(&self) -> usize {
        (self.solution.width() + 1) / 2
    }

    pub fn col_hint(&self, col: usize, pos: usize) -> HintCell {
        self.col_hints[pos][col]
    }

    pub fn row_hint(&self, row: usize, pos: usize) -> HintCell {
        self.row_hints[row][pos]
    }

    pub fn field(&self, row: usize, col: usize) -> FieldCell {
        self.field[row][col]
    }

    pub fn solution(&self, row: usize, col: usize) -> FieldCell {
        self.solution[row][col]
    }

    pub fn field_ref(&self) -> &Grid<FieldCell> {
        &self.field
    }

    pub fn solution_ref(&self) -> &Grid<FieldCell> {
        &self.solution
    }

    pub fn fill(&mut self, row: usize, col: usize) -> bool {
        let cell = &mut self.field[row][col];
        if *cell == FieldCell::Empty {
            *cell = FieldCell::Filled;
            return true;
        }
        false
    }

    /// Mark a cell (as known empty),
    /// return true iff the cell was not previously marked
    pub fn mark(&mut self, row: usize, col: usize) -> bool {
        let cell = &mut self.field[row][col];
        if *cell != FieldCell::Marked {
            *cell = FieldCell::Marked;
            return true;
        }
        false
    }

    /// Remove mark from a cell (leaving it as empty),
    /// return true iff the cell was previously marked,
    /// no-op if the cell was filled or empty
    pub fn unmark(&mut self, row: usize, col: usize) -> bool {
        let cell = &mut self.field[row][col];
        if *cell == FieldCell::Marked {
            *cell = FieldCell::Empty;
            return true;
        }
        false
    }

    /// Set a cell in the solution,
    /// return true iff the solution was changed
    pub fn set(&mut self, row: usize, col: usize, filled: bool) -> bool {
        let cell = &mut self.solution[row][col];
        let target_val = if filled {
            FieldCell::Filled
        } else {
            FieldCell::Empty
        };
        if *cell != target_val {
            *cell = target_val;
            self.generate_hints();
            return true;
        }
        false
    }
}
