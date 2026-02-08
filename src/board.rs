pub const BOARD_WIDTH: usize = 9;
pub const BOARD_HEIGHT: usize = 10;
pub type BoardShape = [[i8; BOARD_HEIGHT]; BOARD_WIDTH];

#[cfg(test)]
use crate::position::Position;
#[cfg(test)]
pub fn generate_board(chesses: Vec<(i8, Position)>) -> BoardShape {
    use crate::chess::{MAX_CHESS_ID, MIN_CHESS_ID};
    let mut board = [[0i8; BOARD_HEIGHT]; BOARD_WIDTH];
    for (id, pos) in chesses {
        assert!(
            MIN_CHESS_ID <= id && id <= MAX_CHESS_ID,
            "invalid id : {}",
            id
        );
        assert!(
            pos.x <= BOARD_WIDTH && pos.y <= BOARD_HEIGHT,
            "invalid pos: {}",
            pos
        );

        board[pos.x][pos.y] = id;
    }

    board
}
