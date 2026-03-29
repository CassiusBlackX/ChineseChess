pub const BOARD_WIDTH: usize = 9;
pub const BOARD_HEIGHT: usize = 10;
pub type BoardShape = [[i8; BOARD_HEIGHT]; BOARD_WIDTH];

use crate::{
    chess::{MAX_CHESS_ID, MIN_CHESS_ID, *},
    position::Position,
    vec2d::Vec2d,
};
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WalkErr {
    OutOfBound,
    Unreachable,
    Hindered,
}

pub struct Board {
    pieces: [Box<dyn ChessTrait>; 32],
    board_status: BoardShape,
}

impl Board {
    fn all_piece_ids() -> [i8; 32] {
        [
            RED_KING_ID,
            RED_LEFT_SERVANT_ID,
            RED_RIGHT_SERVANT_ID,
            RED_LEFT_ELEPHANT_ID,
            RED_RIGHT_ELEPHANT_ID,
            RED_LEFT_HORSE_ID,
            RED_RIGHT_HORSE_ID,
            RED_LEFT_CAR_ID,
            RED_RIGHT_CAR_ID,
            RED_LEFT_CANNON_ID,
            RED_RIGHT_CANNON_ID,
            RED_MIDDLE_PAWN_ID,
            RED_MIDDLE_LEFT_PAWN_ID,
            RED_MIDDLE_RIGHT_PAWN_ID,
            RED_LEFTEST_PAWN_ID,
            RED_RIGHTEST_PAWN_ID,
            BLACK_KING_ID,
            BLACK_LEFT_SERVANT_ID,
            BLACK_RIGHT_SERVANT_ID,
            BLACK_LEFT_ELEPHANT_ID,
            BLACK_RIGHT_ELEPHANT_ID,
            BLACK_LEFT_HORSE_ID,
            BLACK_RIGHT_HORSE_ID,
            BLACK_LEFT_CAR_ID,
            BLACK_RIGHT_CAR_ID,
            BLACK_LEFT_CANNON_ID,
            BLACK_RIGHT_CANNON_ID,
            BLACK_MIDDLE_PAWN_ID,
            BLACK_MIDDLE_LEFT_PAWN_ID,
            BLACK_MIDDLE_RIGHT_PAWN_ID,
            BLACK_LEFTEST_PAWN_ID,
            BLACK_RIGHTEST_PAWN_ID,
        ]
    }

    fn find_piece_pos(board_status: &BoardShape, id: i8) -> Option<Position> {
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                if board_status[x][y] == id {
                    return Some(Position { x, y });
                }
            }
        }
        None
    }

    fn build_piece_with_pos(id: i8, pos: Position) -> Box<dyn ChessTrait> {
        match id {
            RED_KING_ID | BLACK_KING_ID => Box::new(King::new_with_pos(id, pos)),
            RED_LEFT_SERVANT_ID | RED_RIGHT_SERVANT_ID | BLACK_LEFT_SERVANT_ID
            | BLACK_RIGHT_SERVANT_ID => Box::new(Servant::new_with_pos(id, pos)),
            RED_LEFT_ELEPHANT_ID | RED_RIGHT_ELEPHANT_ID | BLACK_LEFT_ELEPHANT_ID
            | BLACK_RIGHT_ELEPHANT_ID => Box::new(Elephant::new_with_pos(id, pos)),
            RED_LEFT_HORSE_ID | RED_RIGHT_HORSE_ID | BLACK_LEFT_HORSE_ID | BLACK_RIGHT_HORSE_ID => {
                Box::new(Horse::new_with_pos(id, pos))
            }
            RED_LEFT_CAR_ID | RED_RIGHT_CAR_ID | BLACK_LEFT_CAR_ID | BLACK_RIGHT_CAR_ID => {
                Box::new(Car::new_with_pos(id, pos))
            }
            RED_LEFT_CANNON_ID | RED_RIGHT_CANNON_ID | BLACK_LEFT_CANNON_ID
            | BLACK_RIGHT_CANNON_ID => Box::new(Cannon::new_with_pos(id, pos)),
            RED_MIDDLE_PAWN_ID
            | RED_MIDDLE_LEFT_PAWN_ID
            | RED_MIDDLE_RIGHT_PAWN_ID
            | RED_LEFTEST_PAWN_ID
            | RED_RIGHTEST_PAWN_ID
            | BLACK_MIDDLE_PAWN_ID
            | BLACK_MIDDLE_LEFT_PAWN_ID
            | BLACK_MIDDLE_RIGHT_PAWN_ID
            | BLACK_LEFTEST_PAWN_ID
            | BLACK_RIGHTEST_PAWN_ID => Box::new(Pawn::new_with_pos(id, pos)),
            _ => panic!("unsupported chess id: {}", id),
        }
    }

    fn build_piece(id: i8, pos: Option<Position>) -> Box<dyn ChessTrait> {
        if let Some(found_pos) = pos {
            return Self::build_piece_with_pos(id, found_pos);
        }

        let mut piece: Box<dyn ChessTrait> = match id {
            RED_KING_ID | BLACK_KING_ID => Box::new(King::new(id)),
            RED_LEFT_SERVANT_ID | RED_RIGHT_SERVANT_ID | BLACK_LEFT_SERVANT_ID
            | BLACK_RIGHT_SERVANT_ID => Box::new(Servant::new(id)),
            RED_LEFT_ELEPHANT_ID | RED_RIGHT_ELEPHANT_ID | BLACK_LEFT_ELEPHANT_ID
            | BLACK_RIGHT_ELEPHANT_ID => Box::new(Elephant::new(id)),
            RED_LEFT_HORSE_ID | RED_RIGHT_HORSE_ID | BLACK_LEFT_HORSE_ID | BLACK_RIGHT_HORSE_ID => {
                Box::new(Horse::new(id))
            }
            RED_LEFT_CAR_ID | RED_RIGHT_CAR_ID | BLACK_LEFT_CAR_ID | BLACK_RIGHT_CAR_ID => {
                Box::new(Car::new(id))
            }
            RED_LEFT_CANNON_ID | RED_RIGHT_CANNON_ID | BLACK_LEFT_CANNON_ID
            | BLACK_RIGHT_CANNON_ID => Box::new(Cannon::new(id)),
            RED_MIDDLE_PAWN_ID
            | RED_MIDDLE_LEFT_PAWN_ID
            | RED_MIDDLE_RIGHT_PAWN_ID
            | RED_LEFTEST_PAWN_ID
            | RED_RIGHTEST_PAWN_ID
            | BLACK_MIDDLE_PAWN_ID
            | BLACK_MIDDLE_LEFT_PAWN_ID
            | BLACK_MIDDLE_RIGHT_PAWN_ID
            | BLACK_LEFTEST_PAWN_ID
            | BLACK_RIGHTEST_PAWN_ID => Box::new(Pawn::new(id)),
            _ => panic!("unsupported chess id: {}", id),
        };
        piece.killed();
        piece
    }

    fn piece_index(id: i8) -> Option<usize> {
        if !(MIN_CHESS_ID..=MAX_CHESS_ID).contains(&id) || id == 0 {
            return None;
        }
        if id > 0 {
            Some(id as usize - 1)
        } else {
            Some((-id) as usize + 15)
        }
    }

    pub fn new() -> Self {
        let pieces: [Box<dyn ChessTrait>; 32] = [
            Box::new(King::new(RED_KING_ID)),
            Box::new(Servant::new(RED_LEFT_SERVANT_ID)),
            Box::new(Servant::new(RED_RIGHT_SERVANT_ID)),
            Box::new(Elephant::new(RED_LEFT_ELEPHANT_ID)),
            Box::new(Elephant::new(RED_RIGHT_ELEPHANT_ID)),
            Box::new(Horse::new(RED_LEFT_HORSE_ID)),
            Box::new(Horse::new(RED_RIGHT_HORSE_ID)),
            Box::new(Car::new(RED_LEFT_CAR_ID)),
            Box::new(Car::new(RED_RIGHT_CAR_ID)),
            Box::new(Cannon::new(RED_LEFT_CANNON_ID)),
            Box::new(Cannon::new(RED_RIGHT_CANNON_ID)),
            Box::new(Pawn::new(RED_MIDDLE_PAWN_ID)),
            Box::new(Pawn::new(RED_MIDDLE_LEFT_PAWN_ID)),
            Box::new(Pawn::new(RED_MIDDLE_RIGHT_PAWN_ID)),
            Box::new(Pawn::new(RED_LEFTEST_PAWN_ID)),
            Box::new(Pawn::new(RED_RIGHTEST_PAWN_ID)),
            Box::new(King::new(BLACK_KING_ID)),
            Box::new(Servant::new(BLACK_LEFT_SERVANT_ID)),
            Box::new(Servant::new(BLACK_RIGHT_SERVANT_ID)),
            Box::new(Elephant::new(BLACK_LEFT_ELEPHANT_ID)),
            Box::new(Elephant::new(BLACK_RIGHT_ELEPHANT_ID)),
            Box::new(Horse::new(BLACK_LEFT_HORSE_ID)),
            Box::new(Horse::new(BLACK_RIGHT_HORSE_ID)),
            Box::new(Car::new(BLACK_LEFT_CAR_ID)),
            Box::new(Car::new(BLACK_RIGHT_CAR_ID)),
            Box::new(Cannon::new(BLACK_LEFT_CANNON_ID)),
            Box::new(Cannon::new(BLACK_RIGHT_CANNON_ID)),
            Box::new(Pawn::new(BLACK_MIDDLE_PAWN_ID)),
            Box::new(Pawn::new(BLACK_MIDDLE_LEFT_PAWN_ID)),
            Box::new(Pawn::new(BLACK_MIDDLE_RIGHT_PAWN_ID)),
            Box::new(Pawn::new(BLACK_LEFTEST_PAWN_ID)),
            Box::new(Pawn::new(BLACK_RIGHTEST_PAWN_ID)),
        ];
        let mut board_status = [[0i8; BOARD_HEIGHT]; BOARD_WIDTH];
        for piece in &pieces {
            let pos = piece.get_pos();
            let id = piece.get_id();
            board_status[pos.x][pos.y] = id;
        }
        Self {
            pieces,
            board_status,
        }
    }

    pub fn get_piece(&self, id: i8) -> Option<&Box<dyn ChessTrait>> {
        Self::piece_index(id).map(|idx| &self.pieces[idx])
    }

    pub fn get_piece_mut(&mut self, id: i8) -> Option<&mut Box<dyn ChessTrait>> {
        let idx = Self::piece_index(id)?;
        Some(&mut self.pieces[idx])
    }

    pub fn board_status(&self) -> &BoardShape {
        &self.board_status
    }

    pub fn id_at(&self, pos: Position) -> i8 {
        self.board_status[pos.x][pos.y]
    }

    pub fn piece_name(&self, id: i8) -> Option<char> {
        self.get_piece(id).map(|piece| piece.get_name())
    }

    pub fn walk_options(&mut self, id: i8) -> &[Option<Position>] {
        let board_status = self.board_status;
        let piece = self.get_piece_mut(id).expect("invalid id");
        let (walk_options, _) = piece.walk_options(&board_status);
        walk_options
    }

    pub fn walk(&mut self, id: i8, target_vec2d: Vec2d) -> Result<(), WalkErr> {
        let piece_idx = Self::piece_index(id).ok_or(WalkErr::Unreachable)?;
        if !self.pieces[piece_idx].is_alive() {
            return Err(WalkErr::Unreachable);
        }

        let cur_pos = self.pieces[piece_idx].get_pos();
        let Some(target_pos) = cur_pos.checked_add_vec2d(target_vec2d) else {
            return Err(WalkErr::OutOfBound);
        };

        let board_status = self.board_status;
        let can_walk = {
            let piece = &mut self.pieces[piece_idx];
            let (options, _) = piece.walk_options(&board_status);
            options.iter().any(|opt| opt.as_ref() == Some(&target_pos))
        };
        if !can_walk {
            return Err(WalkErr::Unreachable);
        }

        let target_id = self.board_status[target_pos.x][target_pos.y];
        if target_id != 0 && same_side(id, target_id) {
            return Err(WalkErr::Hindered);
        }

        if target_id != 0 {
            let target_idx = Self::piece_index(target_id).ok_or(WalkErr::Unreachable)?;
            self.pieces[target_idx].killed();
        }

        let did_walk = self.pieces[piece_idx].walk(target_vec2d);
        if !did_walk {
            return Err(WalkErr::Unreachable);
        }

        self.board_status[cur_pos.x][cur_pos.y] = 0;
        self.board_status[target_pos.x][target_pos.y] = id;
        Ok(())
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let ids = Self::all_piece_ids();
        let pieces = std::array::from_fn(|i| {
            let id = ids[i];
            let pos = Self::find_piece_pos(&self.board_status, id);
            Self::build_piece(id, pos)
        });

        Self {
            pieces,
            board_status: self.board_status,
        }
    }
}

#[cfg(test)]
impl Board {
    pub(crate) fn get_board_status(&self) -> &BoardShape {
        &self.board_status
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pos;

    #[test]
    fn get_chess_test() {
        let board = Board::new();
        let names: [char; 32] = [
            '卒', '卒', '卒', '卒', '卒', '鞄', '鞄', '单', '单', '馬', '馬', '象', '象', '士',
            '士', '将', '帅', '仕', '仕', '相', '相', '马', '马', '车', '车', '炮', '炮', '兵',
            '兵', '兵', '兵', '兵',
        ];
        for (i, id) in (MIN_CHESS_ID..=MAX_CHESS_ID)
            .filter(|&id| id != 0)
            .enumerate()
        {
            let piece = board.get_piece(id).unwrap();
            assert_eq!(piece.get_id(), id);
            assert_eq!(piece.get_name(), names[i]);
        }

        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let id = board.get_board_status()[i][j];
                if id != 0 {
                    let piece = board.get_piece(id).unwrap();
                    assert_eq!(piece.get_pos(), pos!(i, j));
                }
            }
        }
    }
}
