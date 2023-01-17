pub struct DragSelection {
    start: (i32, i32),
    end: (i32, i32),
    current_column: i32,
}

/// Iterator to walk from a square shaped selection in a 2D grid
impl DragSelection {
    pub fn new(corner1: (i32, i32), corner2: (i32, i32)) -> Self {
        let start = (
            std::cmp::min(corner1.0, corner2.0),
            std::cmp::min(corner1.1, corner2.1),
        );
        let end = (
            std::cmp::max(corner1.0, corner2.0),
            std::cmp::max(corner1.1, corner2.1),
        );
        Self {
            start,
            end,
            current_column: start.1,
        }
    }
}

impl Iterator for DragSelection {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start.0 > self.end.0 {
            return None;
        }
        let result = (self.start.0, self.current_column);
        if self.current_column < self.end.1 {
            self.current_column += 1;
        } else {
            self.current_column = self.start.1;
            self.start.0 += 1;
        }
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.end.0 - self.start.0 + 1) as usize
            * (self.end.1 - self.start.1 + 1) as usize;
        (size, Some(size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_walk_over_single_selected_cell() {
        let mut sel = DragSelection::new((5, 5), (5, 5));
        assert_eq!(Some((5, 5)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_walk_over_three_cells_in_a_row() {
        let mut sel = DragSelection::new((5, 5), (5, 7));
        assert_eq!(Some((5, 5)), sel.next());
        assert_eq!(Some((5, 6)), sel.next());
        assert_eq!(Some((5, 7)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_walk_over_cells_in_a_row() {
        let mut sel = DragSelection::new((5, 5), (5, 7));
        assert_eq!(Some((5, 5)), sel.next());
        assert_eq!(Some((5, 6)), sel.next());
        assert_eq!(Some((5, 7)), sel.next());
        assert_eq!(None, sel.next());
        let mut sel = DragSelection::new((5, 7), (5, 5));
        assert_eq!(Some((5, 5)), sel.next());
        assert_eq!(Some((5, 6)), sel.next());
        assert_eq!(Some((5, 7)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_walk_over_cells_in_a_column() {
        let mut sel = DragSelection::new((5, 0), (7, 0));
        assert_eq!(Some((5, 0)), sel.next());
        assert_eq!(Some((6, 0)), sel.next());
        assert_eq!(Some((7, 0)), sel.next());
        assert_eq!(None, sel.next());
        let mut sel = DragSelection::new((7, 0), (5, 0));
        assert_eq!(Some((5, 0)), sel.next());
        assert_eq!(Some((6, 0)), sel.next());
        assert_eq!(Some((7, 0)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_walk_row_by_row() {
        let mut sel = DragSelection::new((0, 0), (4, 3));
        assert_eq!(Some((0, 0)), sel.next());
        assert_eq!(Some((0, 1)), sel.next());
        assert_eq!(Some((0, 2)), sel.next());
        assert_eq!(Some((0, 3)), sel.next());
        assert_eq!(Some((1, 0)), sel.next());
        assert_eq!(Some((1, 1)), sel.next());
        assert_eq!(Some((1, 2)), sel.next());
        assert_eq!(Some((1, 3)), sel.next());
        assert_eq!(Some((2, 0)), sel.next());
        assert_eq!(Some((2, 1)), sel.next());
        assert_eq!(Some((2, 2)), sel.next());
        assert_eq!(Some((2, 3)), sel.next());
        assert_eq!(Some((3, 0)), sel.next());
        assert_eq!(Some((3, 1)), sel.next());
        assert_eq!(Some((3, 2)), sel.next());
        assert_eq!(Some((3, 3)), sel.next());
        assert_eq!(Some((4, 0)), sel.next());
        assert_eq!(Some((4, 1)), sel.next());
        assert_eq!(Some((4, 2)), sel.next());
        assert_eq!(Some((4, 3)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_return_correct_size_hint_for_single_cell() {
        assert_eq!((1, Some(1)), DragSelection::new((0, 0), (0, 0)).size_hint());
    }

    #[test]
    fn should_return_correct_size_hint_for_cells_in_a_row() {
        assert_eq!((4, Some(4)), DragSelection::new((0, 1), (0, 4)).size_hint());
    }

    #[test]
    fn should_return_correct_size_hint_for_cells_in_a_column() {
        assert_eq!((4, Some(4)), DragSelection::new((1, 0), (4, 0)).size_hint());
    }

    #[test]
    fn should_return_correct_size_hint_for_rectangular_selection() {
        assert_eq!(
            (25, Some(25)),
            DragSelection::new((0, 0), (4, 4)).size_hint()
        );
    }
}
