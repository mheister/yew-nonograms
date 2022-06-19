pub struct DragSelection {
    start: (i32, i32),
    end: (i32, i32),
    step: (i32, i32),
}

/// Iterator to walk from a start to an end index pair (inclusive) in a 2D grid. The
/// iterator walks in an L-shape beginning with the dimension of the largest difference
/// between start and end, or the first dimension if they are equal.
impl DragSelection {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let horz_diff = (end.0 - start.0).abs();
        let vert_diff = (end.1 - start.1).abs();
        let step = match (horz_diff - vert_diff).signum() {
            1 => {
                let horz_dir = (end.0 - start.0).signum();
                (horz_dir, 0)
            }
            -1 => {
                let vert_dir = (end.1 - start.1).signum();
                (0, vert_dir)
            }
            _ => (1, 1),
        };
        Self { start, end, step }
    }
}

impl Iterator for DragSelection {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.start;
        let (walk_idx, step, end) = if self.step.0 != 0 {
            (&mut self.start.0, self.step.0, self.end.0)
        } else {
            (&mut self.start.1, self.step.1, self.end.1)
        };
        if (end - *walk_idx).signum() * step != -1 {
            *walk_idx += step;
            if *walk_idx == end {
                // Change direction
                *self = DragSelection::new(self.start, self.end);
            }
            Some(result)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = 1usize
            + (self.end.0 - self.start.0).abs() as usize
            + (self.end.1 - self.start.1).abs() as usize;
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
    fn should_walk_over_three_cells_in_a_row_towards_smaller_indexes() {
        let mut sel = DragSelection::new((5, 7), (5, 5));
        assert_eq!(Some((5, 7)), sel.next());
        assert_eq!(Some((5, 6)), sel.next());
        assert_eq!(Some((5, 5)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_walk_over_three_cells_in_a_column() {
        let mut sel = DragSelection::new((5, 0), (7, 0));
        assert_eq!(Some((5, 0)), sel.next());
        assert_eq!(Some((6, 0)), sel.next());
        assert_eq!(Some((7, 0)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_walk_in_l_shape_beginning_with_bigger_side() {
        let mut sel = DragSelection::new((0, 0), (4, 3));
        assert_eq!(Some((0, 0)), sel.next());
        assert_eq!(Some((1, 0)), sel.next());
        assert_eq!(Some((2, 0)), sel.next());
        assert_eq!(Some((3, 0)), sel.next());
        assert_eq!(Some((4, 0)), sel.next());
        assert_eq!(Some((4, 1)), sel.next());
        assert_eq!(Some((4, 2)), sel.next());
        assert_eq!(Some((4, 3)), sel.next());
        assert_eq!(None, sel.next());
        let mut sel = DragSelection::new((0, 0), (3, 4));
        assert_eq!(Some((0, 0)), sel.next());
        assert_eq!(Some((0, 1)), sel.next());
        assert_eq!(Some((0, 2)), sel.next());
        assert_eq!(Some((0, 3)), sel.next());
        assert_eq!(Some((0, 4)), sel.next());
        assert_eq!(Some((1, 4)), sel.next());
        assert_eq!(Some((2, 4)), sel.next());
        assert_eq!(Some((3, 4)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_walk_in_l_shape_beginning_along_column_if_distances_same() {
        let mut sel = DragSelection::new((0, 0), (3, 3));
        assert_eq!(Some((0, 0)), sel.next());
        assert_eq!(Some((1, 0)), sel.next());
        assert_eq!(Some((2, 0)), sel.next());
        assert_eq!(Some((3, 0)), sel.next());
        assert_eq!(Some((3, 1)), sel.next());
        assert_eq!(Some((3, 2)), sel.next());
        assert_eq!(Some((3, 3)), sel.next());
        assert_eq!(None, sel.next());
    }

    #[test]
    fn should_return_correct_size_hint_for_single_cell() {
        assert_eq!((1, Some(1)), DragSelection::new((0, 0), (0, 0)).size_hint());
    }

    #[test]
    fn should_return_correct_size_hint_for_three_cells_in_a_row() {
        assert_eq!((4, Some(4)), DragSelection::new((0, 1), (0, 4)).size_hint());
    }

    #[test]
    fn should_return_correct_size_hint_for_three_cells_in_a_column() {
        assert_eq!((4, Some(4)), DragSelection::new((1, 0), (4, 0)).size_hint());
    }

    #[test]
    fn should_return_correct_size_hint_for_l_shape_selection() {
        assert_eq!((9, Some(9)), DragSelection::new((0, 0), (4, 4)).size_hint());
    }
}
