use board_engine::{Grid, Player};

pub const BOARD_WIDTH: usize = 15;
pub const BOARD_HEIGHT: usize = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Black,
    White,
}

impl Cell {
    pub fn to_player(self) -> Player {
        match self {
            Cell::Empty => 0,
            Cell::Black => 1,
            Cell::White => -1,
        }
    }

    pub fn from_player(player: Player) -> Self {
        match player {
            1 => Cell::Black,
            -1 => Cell::White,
            _ => Cell::Empty,
        }
    }
}

pub struct Board {
    cells: Grid<Cell>,
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Self {
            cells: self.cells.clone(),
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: Grid::new(BOARD_WIDTH, BOARD_HEIGHT),
        }
    }

    pub fn grid(&self) -> &Grid<Cell> {
        &self.cells
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        matches!(self.cells.get(x, y), Some(Cell::Empty) | None)
    }

    pub fn place(&mut self, x: usize, y: usize, player: Player) {
        self.cells.set(x, y, Cell::from_player(player));
    }

    pub fn is_full(&self) -> bool {
        self.cells
            .iter_coords()
            .all(|(x, y)| !matches!(self.cells.get(x, y), Some(Cell::Empty) | None))
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
