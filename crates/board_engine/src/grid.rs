use crate::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T: Copy + Default> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T: Copy + Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![T::default(); width * height],
        }
    }

    pub fn new_filled(width: usize, height: usize, value: T) -> Self {
        Self {
            width,
            height,
            cells: vec![value; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if self.in_bounds(x, y) {
            Some(self.cells[self.index(x, y)])
        } else {
            None
        }
    }

    pub fn get_pos(&self, pos: Position) -> Option<T> {
        self.get(pos.x, pos.y)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        debug_assert!(self.in_bounds(x, y));
        let idx = self.index(x, y);
        self.cells[idx] = value;
    }

    pub fn set_pos(&mut self, pos: Position, value: T) {
        self.set(pos.x, pos.y, value);
    }

    pub fn cells(&self) -> &[T] {
        &self.cells
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| (x, y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_get_set() {
        let mut grid = Grid::<i8>::new(9, 10);
        grid.set(3, 4, 7);
        assert_eq!(grid.get(3, 4), Some(7));
        assert_eq!(grid.get(9, 0), None);
    }
}
