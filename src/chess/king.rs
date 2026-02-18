use crate::chess::{BLACK_KING_ID, Chess, ChessKind, ChessTrait, RED_KING_ID, same_side};
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

const KING_WALK_DIRECTIONS: [Vec2d; 4] = [vec2d!(1, 0), vec2d!(0, 1), vec2d!(-1, 0), vec2d!(0, -1)];

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

    pub fn new_with_pos(id: i8, pos: Position) -> Self {
        assert!(id == RED_KING_ID || id == BLACK_KING_ID);
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
        let reachable_positions: [Option<Position>; KING_WALK_DIRECTIONS.len()] =
            std::array::from_fn(|i| cur_pos.checked_add_vec2d(KING_WALK_DIRECTIONS[i]));
        let walkable_positions =
            position::intersection_option(&optional_positions, &reachable_positions);

        for pos in walkable_positions {
            let other = board_status[pos.x][pos.y];
            if other == 0 || !same_side(id, other) {
                // other == 0, nobody is here, can walk
                // !same_side, an enemy is here, eat him
                self.0.walk_options[self.0.option_count] = Some(pos);
                self.0.option_count += 1;
            }
            // teammate is here, in our way ,can not walk
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
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::generate_board;
    use std::collections::HashSet;

    fn check_options(expected: &[Position], calculated: &[Option<Position>]) {
        let calculated_set: HashSet<Position> = calculated
            .iter()
            .filter_map(|opt| opt.as_ref())
            .copied()
            .collect();
        for pos in expected {
            assert!(
                calculated_set.contains(&pos),
                "calculated does not contains pos: {}",
                pos
            );
        }
    }

    #[test]
    fn empty() {
        // red king
        let king_id = RED_KING_ID;
        let king_pos = pos!(4, 0);
        let board = generate_board(vec![(king_id, king_pos)]);
        let mut king = King::new(king_id);
        let (walk_options, option_count) = king.walk_options(&board);
        assert_eq!(option_count, 3);
        // only 3 optional positions: (3,0),(4,1),(5,0)
        let expected: [Position; 3] = [pos!(3, 0), pos!(4, 1), pos!(5, 0)];
        check_options(&expected, walk_options);

        // black king
        let king_id = BLACK_KING_ID;
        let king_pos = pos!(4, 9);
        let board = generate_board(vec![(king_id, king_pos)]);
        let mut king = King::new(king_id);
        let (walk_options, option_count) = king.walk_options(&board);
        assert_eq!(option_count, 3);
        // only 3 optional positions: (3,9),(4,8),(5,9)
        let expected: [Position; 3] = [pos!(3, 9), pos!(4, 8), pos!(5, 9)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn unreachable_peices() {
        let king_id = RED_KING_ID;
        let king_pos = pos!(4, 1);
        let board = generate_board(vec![
            (king_id, king_pos),
            (-5, pos!(8, 8)),
            (11, pos!(1, 1)),
            (8, pos!(3, 7)),
        ]);
        let mut king = King::new_with_pos(king_id, king_pos);
        let (walk_options, option_count) = king.walk_options(&board);
        assert_eq!(option_count, 4);
        // only 3 optional positions: (3,1),(4,2),(5,1),(4,0)
        let expected: [Position; 4] = [pos!(3, 1), pos!(4, 2), pos!(5, 1), pos!(4, 0)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn enemy() {
        let king_id = RED_KING_ID;
        let king_pos = pos!(4, 1);
        let board = generate_board(vec![
            (king_id, king_pos),
            (-5, pos!(5, 1)),
            (-11, pos!(3, 0)),
        ]);
        let mut king = King::new_with_pos(king_id, king_pos);
        let (walk_options, option_count) = king.walk_options(&board);
        assert_eq!(option_count, 4);
        // 4 optional positions: (3,1),(4,2),(5,1),(4,0)
        let expected: [Position; 4] = [pos!(3, 1), pos!(4, 2), pos!(5, 1), pos!(4, 0)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn teammate() {
        let king_id = BLACK_KING_ID;
        let king_pos = pos!(4, 7);
        let board = generate_board(vec![
            (king_id, king_pos),
            (-2, pos!(4, 8)), // block the way up!
            (3, pos!(3, 8)),  // no in the king's way
        ]);
        let mut king = King::new_with_pos(king_id, king_pos);
        let (walk_options, option_count) = king.walk_options(&board);
        assert_eq!(option_count, 2);
        // 2 optional positions: (3,7),(4,7)
        let expected: [Position; 2] = [pos!(3, 7), pos!(5, 7)];
        check_options(&expected, walk_options);
    }
}
