use crate::grid::Grid;
use std::{rc::Rc, cell::RefCell};
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

impl Into<u8> for FieldCell {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HintCell {
    pub number: u8,    // 0 represents empty field
    pub crossed: bool, // player can mark hints
}

pub struct Board {
    width: usize,
    field: Rc<RefCell<Grid<FieldCell>>>,
    solution: Grid<FieldCell>,
    col_hints: Grid<HintCell>,
    row_hints: Grid<HintCell>,
    preview_generation: u32,
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
            field: Rc::new(RefCell::new(Grid::new(width, width))),
            solution: Grid::from_flat(width, &Vec::<FieldCell>::from_iter(fields)),
            col_hints: Grid::new(width, hint_len),
            row_hints: Grid::new(hint_len, height),
            preview_generation: 0
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
        self.field.borrow_mut()[row][col]
    }

    pub fn solution(&self, row: usize, col: usize) -> FieldCell {
        self.solution[row][col]
    }

    pub fn field_ref(&self) -> Rc<RefCell<Grid<FieldCell>>> {
        self.field.clone()
    }

    pub fn fill(&mut self, row: usize, col: usize) {
        let cell = &mut self.field.borrow_mut()[row][col];
        if *cell != FieldCell::Filled {
            self.preview_generation += 1;
        }
        *cell = FieldCell::Filled;
    }

    pub fn mark(&mut self, row: usize, col: usize) {
        let cell = &mut self.field.borrow_mut()[row][col];
        if *cell == FieldCell::Filled {
            self.preview_generation += 1;
        }
        *cell = match cell {
            FieldCell::Marked => FieldCell::Empty,
            _ => FieldCell::Marked,
        }
    }

    /// Returns the 'preview generation' of the playing field, which is incremented every
    /// time the set of filled cells changes
    pub fn preview_generation(&self) -> u32 {
        self.preview_generation
    }
}
