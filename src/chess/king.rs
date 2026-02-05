use crate::chess::{BLACK_KING_ID, ChessKind, ChessTrait, Chess, RED_KING_ID};
use crate::position::{self, Position};
use crate::vec2d::Vec2d;

use crate::{pos, vec2d};

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

const KING_WALK_DIRECTIONS: [Vec2d; 5] = [
    vec2d!(0, 0),
    vec2d!(1, 0),
    vec2d!(0, 1),
    vec2d!(-1, 0),
    vec2d!(0, -1),
];

const WALK_OPTIONS_COUNT: usize = 9;

#[derive(Debug, Clone)]
pub struct King(Chess<WALK_OPTIONS_COUNT>);

impl King {
    pub fn new(id: i8) -> Self {
        assert!(id == RED_KING_ID || id == BLACK_KING_ID);
        let pos = if id == RED_KING_ID {
            pos!(4, 0)
        } else {
            pos!(4, 9)
        };
        let name = if id == RED_KING_ID { '帅' } else { '将' };
        Self(Chess::new(ChessKind::King, id, true, pos, name))
    }
}

impl ChessTrait for King {
    fn killed(&mut self) {
        self.0.killed()
    }
    fn is_alive(&self) -> bool {
        self.0.is_alive()
    }
    fn get_name(&self) -> char {
        self.0.get_name()
    }

    fn walk_options<'a>(
        &'a mut self,
        board_status: &crate::board::BoardShape,
    ) -> (&'a [Option<Position>], usize) {
        let id = self.0.id;
        let optional_positions = if id > 0 {
            RED_WALK_OPTIONAL_POSITIONS
        } else {
            BLACK_WALK_OPTIONAL_POSITIONS
        };

        let cur_pos = self.0.pos;
        let reachable_positions: [Position; KING_WALK_DIRECTIONS.len()] =
            std::array::from_fn(|i| cur_pos + KING_WALK_DIRECTIONS[i]);
        let walkable_positions = position::intersection(&optional_positions, &reachable_positions);

        for pos in walkable_positions {
            let other = board_status[pos.x][pos.y];
            if other == 0 {
                self.0.walk_options[self.0.option_count] = Some(*pos);
                self.0.option_count += 1;
            }
        }
        (&self.0.walk_options, self.0.option_count)
    }

    fn walk(&mut self, direction: Vec2d) -> bool {
        let new_pos = self.0.pos + direction;
        for pos in self.0.walk_options {
            if let Some(pos_) = pos
                && pos_ == new_pos
            {
                // can walk
                self.0.pos = new_pos;
                return true;
            }
        }
        return false;
    }
}
