use crate::chess::{
    BLACK_LEFT_SERVANT_ID, BLACK_RIGHT_SERVANT_ID, Chess, ChessKind, ChessTrait,
    RED_LEFT_SERVANT_ID, RED_RIGHT_SERVANT_ID, same_side,
};
use crate::position::{Position, intersection_option};

use crate::vec2d::Vec2d;
use crate::{pos, vec2d};

const RED_WALK_OPTIONAL_POSITIONS: [Position; 5] =
    [pos!(3, 0), pos!(4, 1), pos!(5, 0), pos!(3, 2), pos!(5, 2)];

const BLACK_WALK_OPTIONAL_POSITIONS: [Position; 5] =
    [pos!(3, 9), pos!(4, 8), pos!(5, 9), pos!(3, 7), pos!(5, 7)];

const SERVANT_WALK_DIRECTIONS: [Vec2d; 4] =
    [vec2d!(1, 1), vec2d!(-1, 1), vec2d!(-1, -1), vec2d!(1, -1)];

const WALK_OPTIONS_COUNT: usize = 5;

#[derive(Debug, Clone)]
pub struct Servant(Chess<WALK_OPTIONS_COUNT>);

impl Servant {
    pub fn new(id: i8) -> Self {
        let pos = match id {
            RED_LEFT_SERVANT_ID => pos!(3, 0),
            RED_RIGHT_SERVANT_ID => pos!(5, 0),
            BLACK_LEFT_SERVANT_ID => pos!(3, 9),
            BLACK_RIGHT_SERVANT_ID => pos!(5, 9),
            _ => panic!("invalid id for servant: {}", id),
        };
        let name = if id > 0 { '仕' } else { '士' };
        Self(Chess::new(ChessKind::Servant, id, true, pos, name))
    }

    pub fn new_with_pos(id: i8, pos: Position) -> Self {
        let name = match id {
            RED_LEFT_SERVANT_ID | RED_RIGHT_SERVANT_ID => '仕',
            BLACK_LEFT_SERVANT_ID | BLACK_RIGHT_SERVANT_ID => '士',
            _ => panic!("invalid id for servant: {}", id),
        };
        Self(Chess::new(ChessKind::Servant, id, true, pos, name))
    }
}

impl ChessTrait for Servant {
    fn killed(&mut self) {
        self.0.killed();
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
        let reachable_positions: [Option<Position>; SERVANT_WALK_DIRECTIONS.len()] =
            std::array::from_fn(|i| cur_pos.checked_add_vec2d(SERVANT_WALK_DIRECTIONS[i]));
        let walkable_positions = intersection_option(&optional_positions, &reachable_positions);

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
        // red left servant
        let servant_id = RED_LEFT_SERVANT_ID;
        let servant_pos = pos!(3, 0);
        let board = generate_board(vec![(servant_id, servant_pos)]);
        let mut servant = Servant::new(servant_id);
        let (walk_options, option_count) = servant.walk_options(&board);
        assert_eq!(option_count, 1);
        // only can walk to (4, 1)
        let expected: [Position; 1] = [pos!(4, 1)];
        check_options(&expected, walk_options);

        // black right servant
        let servant_id = BLACK_RIGHT_SERVANT_ID;
        let servant_pos = pos!(5, 9);
        let board = generate_board(vec![(servant_id, servant_pos)]);
        let mut servant = Servant::new(servant_id);
        let (walk_options, option_count) = servant.walk_options(&board);
        assert_eq!(option_count, 1);
        // only can walk to (4, 8)
        let expected: [Position; 1] = [pos!(4, 8)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn unreachable_pieces() {
        let servant_id = RED_RIGHT_SERVANT_ID;
        let servant_pos = pos!(4, 1);
        let board = generate_board(vec![
            (servant_id, servant_pos),
            (5, pos!(2, 0)),
            (-13, pos!(3, 2)),
            (9, pos!(4, 2)),
            (-9, pos!(8, 6)),
        ]);
        let mut servant = Servant::new_with_pos(servant_id, servant_pos);
        let (walk_options, option_count) = servant.walk_options(&board);
        assert_eq!(option_count, 4);
        // 4 options: (3,0),(3,2),(5,0),(5,2)
        let expected: [Position; 4] = [pos!(3, 0), pos!(3, 2), pos!(5, 0), pos!(5, 2)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn enemy() {
        let servant_id = BLACK_LEFT_SERVANT_ID;
        let servant_pos = pos!(5, 7);
        let board = generate_board(vec![
            (servant_id, servant_pos),
            (5, pos!(4, 8)),
            (6, pos!(5, 8)),
            (-8, pos!(6, 8)),
        ]);
        let mut servant = Servant::new_with_pos(servant_id, servant_pos);
        let (walk_options, option_count) = servant.walk_options(&board);
        assert_eq!(option_count, 1);
        // id:5 can be eated, and is the only walk option
        let expected: [Position; 1] = [pos!(4, 8)];
        check_options(&expected, walk_options);
    }

    #[test]
    fn teammate() {
        let servant_id = RED_LEFT_SERVANT_ID;
        let servant_pos = pos!(3, 2);
        let board = generate_board(vec![
            (servant_id, servant_pos),
            (8, pos!(3, 1)),
            (9, pos!(4, 1)),
            (10, pos!(4, 2)),
            (-11, pos!(3, 3)),
        ]);
        let mut servant = Servant::new_with_pos(servant_id, servant_pos);
        let (_, option_count) = servant.walk_options(&board);
        assert_eq!(option_count, 0);
    }
}
