#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
    width: usize,
    cells: Vec<T>,
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
        Self { width, cells: cells.into() }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    #[allow(unused)]
    pub fn height(&self) -> usize {
        if self.width == 0 {
            0
        } else {
            self.cells.len() / self.width
        }
    }
}

impl<T: Default + Clone + Copy + Into<u8>> Grid<T> {
    pub fn serialize_base64(&self) -> String {
        let mut res = format!("{:04x}{:04x}", self.width, self.height());
        self.cells[..].chunks(4).for_each(|chunk| {
            println!("nchunks {}", chunk.len());
            let byte: u8 = chunk
                .iter()
                .map(|elem| Into::<u8>::into(*elem))
                .enumerate()
                .reduce(|(i1, val1), (i2, val2)| {
                    (0, val1 << (2 * i1 as u8) | val2 << (2 * i2 as u8))
                })
                .unwrap_or((0, 0u8))
                .1;
            res += &format!("{:02x}", byte);
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_grid_serializes_without_panic() {
        let _ = Grid::<u8>::new(0, 0).serialize_base64();
        let _ = Grid::<u8>::new(7, 7).serialize_base64();
        let _ = Grid::<u8>::new(7, 0).serialize_base64();
        let _ = Grid::<u8>::new(0, 7).serialize_base64();
        let _ = Grid::<u8>::new(1000, 1000).serialize_base64();
    }

    #[test]
    fn nullsize_grid_serializes_correctly() {
        let serialized = Grid::<u8>::new(0, 0).serialize_base64();
        assert_eq!(serialized, "00000000");
    }

    #[test]
    fn empty_grid_serializes_correctly() {
        let serialized = Grid::<u8>::new(7, 3).serialize_base64();
        assert_eq!(serialized, "00070003000000000000");
    }

    #[test]
    fn single_cell_grid_serializes_correctly() {
        let serialized = Grid::<u8>::from_flat(1, &[0]).serialize_base64();
        assert_eq!(serialized, "0001000100");
        let serialized = Grid::<u8>::from_flat(1, &[1]).serialize_base64();
        assert_eq!(serialized, "0001000101");
        let serialized = Grid::<u8>::from_flat(1, &[2]).serialize_base64();
        assert_eq!(serialized, "0001000102");
        let serialized = Grid::<u8>::from_flat(1, &[3]).serialize_base64();
        assert_eq!(serialized, "0001000103");
    }

    #[test]
    fn four_cell_horizontal_grid_serializes_correctly() {
        let serialized = Grid::<u8>::from_flat(4, &[0; 4]).serialize_base64();
        assert_eq!(serialized, "0004000100");
        let serialized = Grid::<u8>::from_flat(4, &[3; 4]).serialize_base64();
        assert_eq!(serialized, "00040001ff");
        let serialized = Grid::<u8>::from_flat(4, &[0, 0, 0, 1]).serialize_base64();
        assert_eq!(serialized, "0004000140");
        let serialized = Grid::<u8>::from_flat(4, &[0, 1, 2, 3]).serialize_base64();
        assert_eq!(serialized, "00040001e4");
    }

    #[test]
    fn four_cell_vertical_grid_serializes_correctly() {
        let serialized = Grid::<u8>::from_flat(1, &[0; 4]).serialize_base64();
        assert_eq!(serialized, "0001000400");
        let serialized = Grid::<u8>::from_flat(1, &[3; 4]).serialize_base64();
        assert_eq!(serialized, "00010004ff");
        let serialized = Grid::<u8>::from_flat(1, &[0, 0, 0, 1]).serialize_base64();
        assert_eq!(serialized, "0001000440");
        let serialized = Grid::<u8>::from_flat(1, &[0, 1, 2, 3]).serialize_base64();
        assert_eq!(serialized, "00010004e4");
    }

    #[test]
    fn four_cell_square_grid_serializes_correctly() {
        let serialized = Grid::<u8>::from_flat(2, &[0; 4]).serialize_base64();
        assert_eq!(serialized, "0002000200");
        let serialized = Grid::<u8>::from_flat(2, &[3; 4]).serialize_base64();
        assert_eq!(serialized, "00020002ff");
        let serialized = Grid::<u8>::from_flat(2, &[0, 0, 0, 1]).serialize_base64();
        assert_eq!(serialized, "0002000240");
        let serialized = Grid::<u8>::from_flat(2, &[0, 1, 2, 3]).serialize_base64();
        assert_eq!(serialized, "00020002e4");
    }
}
