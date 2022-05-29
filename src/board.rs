use itertools::Itertools;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HintCell {
    pub number: u8,    // 0 represents empty field
    pub crossed: bool, // player can mark hints
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
    width: usize,
    cells: Vec<T>,
}

pub struct Board {
    width: usize,
    field: Grid<FieldCell>,
    solution: Grid<FieldCell>,
    col_hints: Grid<HintCell>,
    row_hints: Grid<HintCell>,
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let rowstart = self.width as usize * index;
        &self.cells[rowstart..rowstart + self.width as usize]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let rowstart = self.width as usize * index;
        &mut self.cells[rowstart..rowstart + self.width as usize]
    }
}

impl<T: Default + Clone + Copy> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            cells: [T::default()].repeat(width * height),
        }
    }
    pub fn from_flat(width: usize, cells: &[T]) -> Self {
        let mut cells: Vec<T> = cells.into();
        cells.resize(width * width, T::default());
        Self { width, cells }
    }
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn hint_len(&self) -> usize {
        (self.solution.width + 1) / 2
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

    pub fn fill(&mut self, row: usize, col: usize) {
        self.field[row][col] = FieldCell::Filled;
    }

    pub fn mark(&mut self, row: usize, col: usize) {
        let cell = &mut self.field[row][col];
        *cell = match cell {
            FieldCell::Marked => FieldCell::Empty,
            _ => FieldCell::Marked
        }
    }
}
