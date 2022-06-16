use itertools::Itertools;

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
        Self {
            width,
            cells: cells.into(),
        }
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

#[derive(Debug)]
pub enum DeserializationError {
    LengthMismatch,
    InvalidBase64,
    Other,
}

impl<T: Default + Clone + Copy + Into<u8> + From<u8>> Grid<T> {
    pub fn serialize_base64(&self) -> String {
        let mut res = String::new();
        let n_bytes_dimensions = 4;
        let height = self.height();
        let n_raw_bytes_content = self.width * height * 2 / 8 + 1;
        let n_bytes_content_upper_estimate = n_raw_bytes_content * 4 / 3 + 4;
        res.reserve(n_bytes_dimensions + n_bytes_content_upper_estimate);
        let dimensions_data = [
            (self.width & 0xFF) as u8,
            (self.width / 0x100) as u8,
            (height & 0xFF) as u8,
            (height / 0x100) as u8,
        ];
        // 4 bytes of data seem to always be padded up to 8, pre-fill with '=' nonetheless
        let mut dimensions_encoded = ['=' as u8; 8];
        base64::encode_config_slice(
            dimensions_data,
            base64::URL_SAFE,
            &mut dimensions_encoded[..],
        );
        dimensions_encoded.iter().for_each(|b| res.push(*b as char));
        if self.width > 0 && height > 0 {
            let content_data = self.cells[..]
                .chunks(4)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|elem| Into::<u8>::into(*elem))
                        .enumerate()
                        .reduce(|(i1, val1), (i2, val2)| {
                            (0, val1 << (2 * i1 as u8) | val2 << (2 * i2 as u8))
                        })
                        .unwrap_or((0, 0u8))
                        .1
                })
                .collect_vec();
            base64::encode_config_buf(content_data, base64::URL_SAFE_NO_PAD, &mut res);
        }
        res
    }

    pub fn from_base64(src: &str) -> Result<Grid<T>, DeserializationError> {
        if src.len() < 8 {
            return Err(DeserializationError::LengthMismatch);
        }
        let (width, height) = deserialize_grid_dimensions(&src[0..8])?;
        let content_src = &src[8..];
        let serialized_len_upper_boundary = (width * height * 2 / 8 + 1) * 4 / 3 + 4;
        if content_src.len() > serialized_len_upper_boundary {
            return Err(DeserializationError::LengthMismatch);
        }
        let result = base64::decode_config(content_src, base64::URL_SAFE_NO_PAD);
        let content = match result {
            Ok(content) => content,
            Err(_) => {
                return Err(DeserializationError::InvalidBase64);
            }
        };
        let mut grid = Grid::new(width, height);
        if width > 0 && height > 0 {
            content.iter()
                .cartesian_product([0, 2, 4, 6usize])
                .map(|(src_byte, pos)| (src_byte >> pos) & 0b11)
                .take(width * height) // last byte may have <4 cells
                .chunks(width)
                .into_iter()
                .enumerate()
                .for_each(|(row_idx, row)| {
                    row.enumerate().for_each(|(col_idx, cell)| {
                        grid[row_idx][col_idx] = T::from(cell);
                    });
                });
            // content
            //     .iter()
            //     .chunks(width)
            //     .into_iter()
            //     .enumerate()
            //     .for_each(|(row_idx, row)| {
            //         row.enumerate().for_each(|(col_idx, &cell)| {
            //             grid[row_idx][col_idx] = T::from(cell);
            //         });
            //     });
        }
        Ok(grid)
    }
}

fn deserialize_grid_dimensions(
    encoded: &str,
) -> Result<(usize, usize), DeserializationError> {
    // 4 interesting bytes, 8 wide buffer to increase our chance to fail gracefully in
    // case of invalid input
    let mut raw = [0u8; 8];
    let result = base64::decode_config_slice(encoded, base64::URL_SAFE, &mut raw[..]);
    match result {
        Ok(4) => Ok((
            raw[0] as usize + 0x100usize * raw[1] as usize,
            raw[2] as usize + 0x100usize * raw[3] as usize,
        )),
        Ok(_) => Err(DeserializationError::Other),
        Err(_) => Err(DeserializationError::InvalidBase64),
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
        assert_eq!(serialized, "AAAAAA==");
    }

    #[test]
    fn empty_grid_serializes_correctly() {
        let serialized = Grid::<u8>::new(7, 3).serialize_base64();
        assert_eq!(serialized, "BwADAA==AAAAAAAA");
    }

    #[test]
    fn single_cell_grid_serializes_correctly() {
        let serialized = Grid::<u8>::from_flat(1, &[0]).serialize_base64();
        assert_eq!(serialized, "AQABAA==AA");
        let serialized = Grid::<u8>::from_flat(1, &[1]).serialize_base64();
        assert_eq!(serialized, "AQABAA==AQ");
        let serialized = Grid::<u8>::from_flat(1, &[2]).serialize_base64();
        assert_eq!(serialized, "AQABAA==Ag");
        let serialized = Grid::<u8>::from_flat(1, &[3]).serialize_base64();
        assert_eq!(serialized, "AQABAA==Aw");
    }

    #[test]
    fn four_cell_horizontal_grid_serializes_correctly() {
        let serialized = Grid::<u8>::from_flat(4, &[0; 4]).serialize_base64();
        assert_eq!(serialized, "BAABAA==AA");
        let serialized = Grid::<u8>::from_flat(4, &[3; 4]).serialize_base64();
        assert_eq!(serialized, "BAABAA==_w");
        let serialized = Grid::<u8>::from_flat(4, &[0, 0, 0, 1]).serialize_base64();
        assert_eq!(serialized, "BAABAA==QA");
        let serialized = Grid::<u8>::from_flat(4, &[0, 1, 2, 3]).serialize_base64();
        assert_eq!(serialized, "BAABAA==5A");
    }

    #[test]
    fn four_cell_vertical_grid_serializes_correctly() {
        let serialized = Grid::<u8>::from_flat(1, &[0; 4]).serialize_base64();
        assert_eq!(serialized, "AQAEAA==AA");
        let serialized = Grid::<u8>::from_flat(1, &[3; 4]).serialize_base64();
        assert_eq!(serialized, "AQAEAA==_w");
        let serialized = Grid::<u8>::from_flat(1, &[0, 0, 0, 1]).serialize_base64();
        assert_eq!(serialized, "AQAEAA==QA");
        let serialized = Grid::<u8>::from_flat(1, &[0, 1, 2, 3]).serialize_base64();
        assert_eq!(serialized, "AQAEAA==5A");
    }

    #[test]
    fn four_cell_square_grid_serializes_correctly() {
        let serialized = Grid::<u8>::from_flat(2, &[0; 4]).serialize_base64();
        assert_eq!(serialized, "AgACAA==AA");
        let serialized = Grid::<u8>::from_flat(2, &[3; 4]).serialize_base64();
        assert_eq!(serialized, "AgACAA==_w");
        let serialized = Grid::<u8>::from_flat(2, &[0, 0, 0, 1]).serialize_base64();
        assert_eq!(serialized, "AgACAA==QA");
        let serialized = Grid::<u8>::from_flat(2, &[0, 1, 2, 3]).serialize_base64();
        assert_eq!(serialized, "AgACAA==5A");
    }

    #[test]
    fn nullsize_grid_deserializes_correctly() {
        let grid = Grid::<u8>::from_base64("AAAAAA==").unwrap();
        assert_eq!(grid.width(), 0);
        assert_eq!(grid.height(), 0);
    }

    #[test]
    fn empty_grid_deserializes_correctly() {
        let serialized = "BwADAA==AAAAAAAA";
        let grid = Grid::<u8>::from_base64(serialized).unwrap();
        assert_eq!(grid.width(), 7);
        assert_eq!(grid.height(), 3);
        for row in 0..3 {
            for col in 0..7 {
                assert_eq!(grid[row][col], 0);
            }
        }
    }

    #[test]
    fn four_cell_horizontal_grid_deserializes_correctly() {
        let grid = Grid::<u8>::from_base64("BAABAA==AA").unwrap();
        for col in 0..4 {
            assert!(grid[0][col] == 0);
        }
        let grid = Grid::<u8>::from_base64("BAABAA==_w").unwrap();
        for col in 0..4 {
            assert_eq!(grid[0][col], 3);
        }
        let grid = Grid::<u8>::from_base64("BAABAA==QA").unwrap();
        for (col, &val) in [0, 0, 0, 1].iter().enumerate() {
            assert_eq!(grid[0][col], val);
        }
        let grid = Grid::<u8>::from_base64("BAABAA==5A").unwrap();
        for (col, &val) in [0, 1, 2, 3].iter().enumerate() {
            assert_eq!(grid[0][col], val);
        }
    }

    #[test]
    fn example_grid_should_reproduce_after_serialization_and_deserialization() {
        let width = 1000;
        let height = width;
        let cells = (0..width * height).map(|i| {
            if i % 77 == 0 {
                return 2;
            }
            if i % 3 == 0 || i % 7 == 0 || i % 11 == 0 {
                1u8
            } else {
                0u8
            }
        }).collect_vec();
        let grid = Grid::<u8>::from_flat(width, &cells);
        assert_eq!(grid, Grid::<u8>::from_base64(&grid.serialize_base64()).unwrap());
    }
}
