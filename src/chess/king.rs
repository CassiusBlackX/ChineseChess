use crate::chess::{BLACK_KING_ID, ChessCommon, ChessKind, ChessTrait, Piece, RED_KING_ID};
use crate::position::Position;

use crate::pos;
use crate::vec2d::Vec2d;

const RED_WALK_OPTIONAL_POSITIONS: [Position; 9] = [
    pos!(3, 0),
    pos!(4, 0),
    pos!(5, 0),
    pos!(3, 1),
    pos!(4, 1),
    pos!(5, 1),
    pos!(3, 2),
    pos!(4, 2),
    pos!(5, 2),
];

const BLACK_WALK_OPTIONAL_POSITIONS: [Position; 9] = [
    pos!(3, 9),
    pos!(4, 9),
    pos!(5, 9),
    pos!(3, 8),
    pos!(4, 8),
    pos!(5, 8),
    pos!(3, 7),
    pos!(4, 7),
    pos!(5, 7),
];

const WALK_OPTIONS_COUNT: usize = 9;

#[derive(Debug, Clone)]
pub struct King(Piece<WALK_OPTIONS_COUNT>);

impl King {
    pub fn new(id: i8) -> Self {
        assert!(id == RED_KING_ID || id == BLACK_KING_ID);
        let pos = if id == RED_KING_ID {
            pos!(4, 0)
        } else {
            pos!(4, 9)
        };
        let name = if id == RED_KING_ID { '帅' } else { '将' };
        Self(Piece {
            common: ChessCommon::new(ChessKind::King, id, true, pos, name),
            walk_options: [None; WALK_OPTIONS_COUNT],
            option_count: 0,
        })
    }
}

impl ChessTrait for King {
    fn killed(&mut self) {
        self.0.common.killed()
    }
    fn is_alive(&self) -> bool {
        self.0.common.is_alive()
    }
    fn get_name(&self) -> char {
        self.0.common.get_name()
    }

    fn walk_options<'a>(
        &'a mut self,
        board_status: &crate::board::BoardShape,
    ) -> (&'a [Option<Position>], usize) {
        let id = self.0.common.id;
        let optional_positions = if id > 0 {
            RED_WALK_OPTIONAL_POSITIONS
        } else {
            BLACK_WALK_OPTIONAL_POSITIONS
        };

        let cur_pos = self.0.common.pos;
        self.0.walk_options[0] = Some(cur_pos);
        self.0.option_count = 1;
        for pos in optional_positions {
            let other = board_status[pos.x][pos.y];
            if other == 0 {
                self.0.walk_options[self.0.option_count] = Some(pos);
                self.0.option_count += 1;
            }
        }
        (&self.0.walk_options, self.0.option_count)
    }

    fn walk(&mut self, direction: Vec2d) -> bool {
        let new_pos = self.0.common.pos + direction;
        for pos in self.0.walk_options {
            if let Some(pos_) = pos {
                if pos_ == new_pos {
                    // can walk
                    self.0.common.pos = new_pos;
                    return true;
                }
            }
        }
        return false;
    }
}
