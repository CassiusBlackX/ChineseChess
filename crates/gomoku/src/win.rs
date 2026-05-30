use board_engine::{Grid, Player, Position};

use crate::board::{Cell, Board};

pub fn check_winner(board: &Grid<Cell>, pos: Position) -> Option<Player> {
    let stone = board.get_pos(pos)?;
    if stone == Cell::Empty {
        return None;
    }
    let player = stone.to_player();

    for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
        let count = 1
            + count_dir(board, pos, dx, dy, stone)
            + count_dir(board, pos, -dx, -dy, stone);
        if count >= 5 {
            return Some(player);
        }
    }
    None
}

fn count_dir(board: &Grid<Cell>, pos: Position, dx: i8, dy: i8, stone: Cell) -> usize {
    let mut count = 0;
    let mut x = pos.x as i8 + dx;
    let mut y = pos.y as i8 + dy;
    while x >= 0
        && y >= 0
        && (x as usize) < board.width()
        && (y as usize) < board.height()
        && board.get(x as usize, y as usize) == Some(stone)
    {
        count += 1;
        x += dx;
        y += dy;
    }
    count
}

pub fn check_winner_on_board(board: &Board, pos: Position) -> Option<Player> {
    check_winner(board.grid(), pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{board::BOARD_WIDTH, pos};

    fn line_board(coords: &[(usize, usize)], stone: Cell) -> Grid<Cell> {
        let mut grid = Grid::new(BOARD_WIDTH, BOARD_WIDTH);
        for &(x, y) in coords {
            grid.set(x, y, stone);
        }
        grid
    }

    #[test]
    fn horizontal_five_wins() {
        let grid = line_board(&[(0, 7), (1, 7), (2, 7), (3, 7), (4, 7)], Cell::Black);
        assert_eq!(check_winner(&grid, pos!(4, 7)), Some(1));
    }

    #[test]
    fn vertical_five_wins() {
        let grid = line_board(&[(5, 0), (5, 1), (5, 2), (5, 3), (5, 4)], Cell::White);
        assert_eq!(check_winner(&grid, pos!(5, 4)), Some(-1));
    }

    #[test]
    fn diagonal_five_wins() {
        let grid = line_board(&[(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)], Cell::Black);
        assert_eq!(check_winner(&grid, pos!(4, 4)), Some(1));
    }

    #[test]
    fn four_in_row_not_win() {
        let grid = line_board(&[(0, 0), (1, 0), (2, 0), (3, 0)], Cell::Black);
        assert_eq!(check_winner(&grid, pos!(3, 0)), None);
    }
}
